///
/// References used:
/// https://wiki.multimedia.cx/index.php?title=Smacker
/// https://github.com/lu-zero/ffmpeg/blob/master/libavcodec/smacker.c
/// https://github.com/jewalky/UnityAllods/blob/smack-support/Assets/SmackLoader.cs
///

use {
    super::{
        huffman::HeaderTree,
        header::SmackerFileHeader,
        decode_context::SmackerDecodeContext,
        frame::SmackerFrameInfo,
        bit_reader::with_bit_reader,
        *
    },
    crate::shared_types::{U32Wrapper, U8Wrapper},
    bin_serialization_rs::{Reflectable, Endianness},
    bitflags::_core::ops::Range,
    std::{
        io::{Read, Seek, Cursor, SeekFrom},
        cmp::Ordering
    }
};

const BLOCK_MONOCHROME: u16 = 0;
const BLOCK_FULL: u16 = 1;
const BLOCK_VOID: u16 = 2;
const BLOCK_SOLID: u16 = 3;

const BLOCK_SIZE_TABLE: &[usize] = &[
    0x001, 0x002, 0x003, 0x004, 0x005, 0x006, 0x007, 0x008,
    0x009, 0x00A, 0x00B, 0x00C, 0x00D, 0x00E, 0x00F, 0x010,
    0x011, 0x012, 0x013, 0x014, 0x015, 0x016, 0x017, 0x018,
    0x019, 0x01A, 0x01B, 0x01C, 0x01D, 0x01E, 0x01F, 0x020,
    0x021, 0x022, 0x023, 0x024, 0x025, 0x026, 0x027, 0x028,
    0x029, 0x02A, 0x02B, 0x02C, 0x02D, 0x02E, 0x02F, 0x030,
    0x031, 0x032, 0x033, 0x034, 0x035, 0x036, 0x037, 0x038,
    0x039, 0x03A, 0x03B, 0x080, 0x100, 0x200, 0x400, 0x800
];

const PALETTE_MAP_TABLE: &[u32] = &[
    0x00, 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18, 0x1C,
    0x20, 0x24, 0x28, 0x2C, 0x30, 0x34, 0x38, 0x3C,
    0x41, 0x45, 0x49, 0x4D, 0x51, 0x55, 0x59, 0x5D,
    0x61, 0x65, 0x69, 0x6D, 0x71, 0x75, 0x79, 0x7D,
    0x82, 0x86, 0x8A, 0x8E, 0x92, 0x96, 0x9A, 0x9E,
    0xA2, 0xA6, 0xAA, 0xAE, 0xB2, 0xB6, 0xBA, 0xBE,
    0xC3, 0xC7, 0xCB, 0xCF, 0xD3, 0xD7, 0xDB, 0xDF,
    0xE3, 0xE7, 0xEB, 0xEF, 0xF3, 0xF7, 0xFB, 0xFF
];

struct FrameBytesShared {
    data: Vec<u8>
}
impl FrameBytesShared {
    fn get_slice(&self, range: Range<usize>) -> &[u8] {
        &self.data[range.start..range.end]
    }
}

pub struct SmackerFileInfo {
    pub width: u32,
    pub height: u32,
    pub frame_interval: f32,
    pub sample_rate_per_frame: usize,
    pub audio_rate: [u32; 7],
    pub audio_flags: [flags::Audio; 7],
    pub audio_tracks: Vec<Vec<f32>>,
    pub smacker_decode_context: SmackerDecodeContext,
    pub frames: Vec<SmackerFrameInfo>,
    m_map_tree: Option<HeaderTree>,
    m_clr_tree: Option<HeaderTree>,
    full_tree: Option<HeaderTree>,
    type_tree: Option<HeaderTree>,
    buffer: Vec<u8>
}
impl SmackerFileInfo {
    fn load(stream: &mut Cursor<&[u8]>) -> std::io::Result<(Self, FrameBytesShared)> {
        let mut header = SmackerFileHeader::deserialize(stream, Endianness::LittleEndian)?;
        let header_flags = flags::Header::from_bits(header.header_flags as u8).unwrap();
        if header_flags.contains(flags::Header::HAS_RING_FRAME) {
            header.num_frames += 1;
        }

        let frame_interval = match header.frame_rate.cmp(&0) {
            Ordering::Less => -header.frame_rate as f32 / 100.0,
            Ordering::Equal => 100.0,
            Ordering::Greater => header.frame_rate as f32
        };
        let mut audio_flags = [Default::default(); 7];
        let mut audio_rate = header.audio_rate.clone();
        for i in 0..7 {
            audio_flags[i] = flags::Audio::from_bits(audio_rate[i] & 0xFC_000000).unwrap();
            audio_rate[i] &= 0x00_FFFFFF;
        }
        let num_frames = header.num_frames as usize;
        let mut frame_sizes = Vec::with_capacity(num_frames);
        let mut frame_flags = Vec::with_capacity(num_frames);
        for _ in 0..num_frames {
            let size = U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
            frame_sizes.push(size.0 & 0xFFFFFFF_C);
            frame_flags.push(flags::Frame::from_bits((size.0 & 0x0000000_3) as u8).unwrap());
        }
        let mut frame_feature_flags = Vec::with_capacity(num_frames);
        for _ in 0..num_frames {
            let frame_type_flag_entry = U8Wrapper::deserialize(stream, Endianness::LittleEndian)?;
            frame_feature_flags.push(flags::FrameFeature::from_bits(frame_type_flag_entry.0).unwrap());
        }

        let trees_start_position = stream.position();
        let mut m_map_tree = None;
        let mut m_clr_tree = None;
        let mut full_tree = None;
        let mut type_tree = None;

        with_bit_reader(stream, |bit_reader| {
            if bit_reader.read_bits(1)? == 1 {
                m_map_tree = Some(HeaderTree::read(bit_reader, header.m_map_size as usize)?);
            }
            if bit_reader.read_bits(1)? == 1 {
                m_clr_tree = Some(HeaderTree::read(bit_reader, header.m_clr_size as usize)?);
            }
            if bit_reader.read_bits(1)? == 1 {
                full_tree = Some(HeaderTree::read(bit_reader, header.full_size as usize)?);
            }
            if bit_reader.read_bits(1)? == 1 {
                type_tree = Some(HeaderTree::read(bit_reader, header.type_size as usize)?);
            }
            Ok(())
        })?;

        stream.seek(SeekFrom::Start(trees_start_position + header.trees_size as u64))?;
        let mut buffer = vec![0u8; 0x10000000];
        let mut frame_bytes_shared = Vec::new();
        let mut frames: Vec<SmackerFrameInfo> = Vec::with_capacity(num_frames);
        for i in 0..num_frames {
            let frame_bytes = &mut buffer[..frame_sizes[i] as usize];
            stream.read(frame_bytes)?;

            let prev_len = frame_bytes_shared.len();
            frame_bytes_shared.extend_from_slice(frame_bytes);
            let frame_range = prev_len..frame_bytes_shared.len();

            frames.push(
                SmackerFrameInfo {
                    frame_range,
                    frame_flags: frame_flags[i],
                    frame_feature_flags: frame_feature_flags[i],
                }
            );
        }

        let width = header.width;
        let height = header.height;

        let smacker_decode_context = SmackerDecodeContext::new(width, height);

        let mut audio_tracks: Vec<Vec<f32>> = vec![Vec::new(); 7];
        for i in 0..7 {
            audio_tracks[i].reserve(header.audio_size[i] as usize);
        }
        let sample_rate_per_frame = (audio_rate[0] as f32 * frame_interval / 1000.0).trunc() as usize;

        Ok(
            (
                Self {
                    width,
                    height,
                    frame_interval,
                    m_map_tree,
                    m_clr_tree,
                    full_tree,
                    type_tree,
                    smacker_decode_context,
                    frames,
                    audio_rate,
                    audio_flags,
                    audio_tracks,
                    sample_rate_per_frame,
                    buffer
                },
                FrameBytesShared{
                    data: frame_bytes_shared
                }
            )
        )
    }

    fn unpack(&mut self, frame_bytes_shared: &FrameBytesShared, frame_id: usize, skip_video: bool, skip_audio: bool) -> std::io::Result<()> {
        let frame_bytes = frame_bytes_shared.get_slice(self.frames[frame_id].frame_range.clone());
        self.unpack_impl(frame_id, frame_bytes, skip_video, skip_audio)
    }

    fn unpack_impl(
        &mut self,
        frame_id: usize,
        frame_bytes: &[u8],
        skip_video: bool,
        skip_audio: bool
    ) -> std::io::Result<()> {
        let frame = self.frames[frame_id].clone();
        if let Some(m_map_tree) = &mut self.m_map_tree {
            m_map_tree.reset_last();
        }
        if let Some(m_clr_tree) = &mut self.m_clr_tree {
            m_clr_tree.reset_last();
        }
        if let Some(full_tree) = &mut self.full_tree {
            full_tree.reset_last();
        }
        if let Some(type_tree) = &mut self.type_tree {
            type_tree.reset_last();
        }
        let mut stream = Cursor::new(frame_bytes);
        if frame.frame_flags.contains(flags::Frame::KEYFRAME) {
            for i in 0..256 {
                self.smacker_decode_context.palette[i] = 0;
            }
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_PALETTE) {
            let palette_size = U8Wrapper::deserialize(&mut stream, Endianness::LittleEndian)?;
            let palette_size = (palette_size.0 as usize) * 4 - 1;
            if !skip_video {
                let pal_colors_buffer = &mut self.buffer[..palette_size];
                stream.read(pal_colors_buffer)?;
                let prev_palette = &self.smacker_decode_context.palette;
                let mut next_palette = prev_palette.clone();
                let (mut offset, mut pal_offset) = (0, 0);
                while offset < pal_colors_buffer.len() && pal_offset < 256 {
                    let flag_byte = pal_colors_buffer[offset];
                    offset += 1;
                    if (flag_byte & 0x80) != 0 {
                        let increment = ((flag_byte & 0x7F) + 1) as usize;
                        pal_offset += increment;
                    } else if (flag_byte & 0xC0) == 0x40 {
                        let prev_pal_offset = pal_colors_buffer[offset] as usize;
                        offset += 1;
                        let increment = ((flag_byte & 0x3F) + 1) as usize;
                        for i in 0..increment {
                            next_palette[pal_offset + i] = prev_palette[prev_pal_offset + i]
                        }
                        pal_offset += increment;
                    } else {
                        let r = PALETTE_MAP_TABLE[(flag_byte & 0x3F) as usize];
                        let flag_byte = pal_colors_buffer[offset];
                        offset += 1;
                        let g = PALETTE_MAP_TABLE[(flag_byte & 0x3F) as usize];
                        let flag_byte = pal_colors_buffer[offset];
                        offset += 1;
                        let b = PALETTE_MAP_TABLE[(flag_byte & 0x3F) as usize];

                        next_palette[pal_offset] =
                            0xFF_000000 + b as u32 + g as u32 * 0x100 + r as u32 * 0x10000;
                        pal_offset += 1;
                    }
                }
                self.smacker_decode_context.palette = next_palette;
            } else {
                stream.seek(SeekFrom::Current(palette_size as i64))?;
            }
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_1) {
            self.unpack_audio(&mut stream, frame_id, 0, skip_audio)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_2) {
            self.unpack_audio(&mut stream, frame_id, 1, skip_audio)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_3) {
            self.unpack_audio(&mut stream, frame_id, 2, skip_audio)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_4) {
            self.unpack_audio(&mut stream, frame_id, 3, skip_audio)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_5) {
            self.unpack_audio(&mut stream, frame_id, 4, skip_audio)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_6) {
            self.unpack_audio(&mut stream, frame_id, 5, skip_audio)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_7) {
            self.unpack_audio(&mut stream, frame_id, 6, skip_audio)?;
        }
        if !skip_video {
            self.unpack_video(&mut stream)?;
        }
        Ok(())
    }

    fn unpack_audio(
        &mut self,
        stream: &mut Cursor<&[u8]>,
        frame_id: usize,
        track_number: usize,
        skip_audio: bool,
    ) -> std::io::Result<()> {
        let _frame_id = frame_id;
        if !self.audio_flags[track_number].contains(flags::Audio::PRESENT) {
            return Ok(())
        }
        let audio_length = U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
        if skip_audio {
            stream.seek(SeekFrom::Current(audio_length.0 as i64 - 4))?;
            Ok(())
        } else {
            unimplemented!()
        }
    }

    fn unpack_video(
        &mut self,
        stream: &mut Cursor<&[u8]>
    ) -> std::io::Result<()> {
        let width_blocks = (self.width / 4) as usize;
        let height_blocks = (self.height / 4) as usize;
        let count_blocks = width_blocks * height_blocks;
        with_bit_reader(stream, |bit_reader| {
            let mut current_block = 0;
            while current_block < count_blocks {
                let mut type_descriptor = self.type_tree.as_mut()
                    .unwrap()
                    .get_value(bit_reader)? as u16;

                let block_type = type_descriptor & 0b11;
                type_descriptor >>= 2;
                let chain_length_idx = (type_descriptor & 0b111111) as usize;
                let chain_length = BLOCK_SIZE_TABLE[chain_length_idx];
                let extra = (type_descriptor >> 6) as u8;
                match block_type {
                    BLOCK_VOID => {
                        current_block += chain_length;
                    },
                    BLOCK_SOLID => {
                        for _ in 0..chain_length {
                            if current_block >= count_blocks {
                                break;
                            }
                            let (x, y) = (
                                (current_block % width_blocks) * 4,
                                (current_block / width_blocks) * 4
                            );
                            let mut stride = y * self.width as usize + x;
                            for _ in 0..4 {
                                for i in stride..stride+4 {
                                    self.smacker_decode_context.image[i] = extra;
                                }
                                stride += self.width as usize;
                            }
                            current_block += 1;
                        }
                    },
                    BLOCK_MONOCHROME => {
                        for _ in 0..chain_length {
                            if current_block >= count_blocks {
                                break;
                            }
                            let color_indices = match self.m_clr_tree.as_mut() {
                                Some(tree) => {
                                    let color_idx_pair = tree.get_value(bit_reader)? as u16;
                                    [
                                        (color_idx_pair & 0xFF) as u8,
                                        (color_idx_pair / 0x100) as u8,
                                    ]
                                },
                                None => unreachable!()
                            };
                            let mut pix_kind_lookup = self.m_map_tree.as_mut()
                                .unwrap()
                                .get_value(bit_reader)?;

                            let (x, y) = (
                                (current_block % width_blocks) * 4,
                                (current_block / width_blocks) * 4
                            );
                            let mut stride = y * self.width as usize + x;
                            for _ in 0..4 {
                                for i in stride..stride+4 {
                                    let kind = (pix_kind_lookup & 0b1) as usize;
                                    pix_kind_lookup >>= 1;
                                    self.smacker_decode_context.image[i] = color_indices[kind];
                                }
                                stride += self.width as usize;
                            }
                            current_block += 1;
                        }
                    }
                    BLOCK_FULL => {
                        for _ in 0..chain_length {
                            if current_block >= count_blocks {
                                break;
                            }
                            let (x, y) = (
                                (current_block % width_blocks) * 4,
                                (current_block / width_blocks) * 4
                            );
                            let mut stride = y * self.width as usize + x;
                            for _ in 0..4 {
                                let color_indices = match self.full_tree.as_mut() {
                                    Some(tree) => {
                                        let color_idx_pair1 = tree.get_value(bit_reader)? as u16;
                                        let color_idx_pair0 = tree.get_value(bit_reader)? as u16;
                                        [
                                            (color_idx_pair0 & 0xFF) as u8,
                                            (color_idx_pair0 / 0x100) as u8,
                                            (color_idx_pair1 & 0xFF) as u8,
                                            (color_idx_pair1 / 0x100) as u8,
                                        ]
                                    },
                                    _ => unreachable!()
                                };
                                for i in 0..4 {
                                    self.smacker_decode_context.image[stride + i] = color_indices[i];
                                }
                                stride += self.width as usize;
                            }
                            current_block += 1;
                        }
                    },
                    _ => unreachable!()
                }
            }
            Ok(())
        })
    }
}

pub struct SmackerFile {
    pub file_info: SmackerFileInfo,
    frame_bytes_shared: FrameBytesShared
}
impl SmackerFile {
    pub fn load(stream: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let (file_info, frame_bytes_shared) = SmackerFileInfo::load(stream)?;
        Ok(Self{ file_info, frame_bytes_shared })
    }
    pub fn unpack(&mut self, frame_id: usize, skip_video: bool, skip_audio: bool) -> std::io::Result<()> {
        self.file_info.unpack(&self.frame_bytes_shared, frame_id, skip_video, skip_audio)
    }
}