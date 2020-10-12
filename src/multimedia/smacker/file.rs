use {
    super::{
        huffman::HeaderTree,
        header::SmackerFileHeader,
        decode_context::SmackerDecodeContext,
        frame::SmackerFrame,
        bit_reader::with_bit_reader,
        *
    },
    crate::shared_types::{U32Wrapper, U8Wrapper},
    bin_serialization_rs::{Reflectable, Endianness},
    std::{
        io::{Read, Seek, Cursor, SeekFrom},
        cmp::Ordering
    }
};

const BLOCK_SIZE_TABLE: &[u32] = &[
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

use crate::multimedia::smacker::flags::Audio;

pub struct SmackerFile {
    pub width: u32,
    pub height: u32,
    pub frame_interval: i32,
    m_map_tree: Option<HeaderTree>,
    m_clr_tree: Option<HeaderTree>,
    full_tree: Option<HeaderTree>,
    type_tree: Option<HeaderTree>,
    pub smacker_decode_context: SmackerDecodeContext,
    pub frames: Vec<SmackerFrame>,
    buffer: Vec<u8>
}
impl SmackerFile {
    pub fn load(stream: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let header = SmackerFileHeader::deserialize(stream, Endianness::LittleEndian)?;
        let frame_interval = match header.frame_rate.cmp(&0) {
            Ordering::Less => -header.frame_rate / 100,
            Ordering::Equal => 100,
            Ordering::Greater => header.frame_rate
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

        let mut frames: Vec<SmackerFrame> = Vec::with_capacity(num_frames);
        for i in 0..num_frames {
            let mut frame_bytes = Vec::with_capacity(frame_sizes[i] as usize);
            stream.read(&mut frame_bytes)?;
            frames.push(
                SmackerFrame {
                    frame_bytes,
                    frame_flags: frame_flags[i],
                    frame_feature_flags: frame_feature_flags[i],
                    audio_flags,
                    audio_rate
                }
            );
        }

        let width = header.width;
        let height = header.height;

        let smacker_decode_context = SmackerDecodeContext::new(width, height);

        Ok(Self {
            width,
            height,
            frame_interval,
            m_map_tree,
            m_clr_tree,
            full_tree,
            type_tree,
            smacker_decode_context,
            frames,
            buffer: vec![0u8; 0x80000]
        })
    }

    pub fn unpack(&mut self, frame_id: usize, frame: &SmackerFrame, unpack_video: bool) -> std::io::Result<()> {
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
        let mut stream = Cursor::new(&frame.frame_bytes[..]);
        if frame.frame_flags.contains(flags::Frame::KEYFRAME) {
            for i in 0..256 {
                self.smacker_decode_context.palette[i] = 0;
            }
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_PALETTE) {
            let palette_size = U8Wrapper::deserialize(&mut stream, Endianness::LittleEndian)?;
            let palette_size = (palette_size.0 as usize) * 4 - 1;
            if unpack_video {
                let mut pal_colors_buffer = &mut self.buffer[..palette_size];
                stream.read(pal_colors_buffer)?;
                let prev_palette = &self.smacker_decode_context.palette;
                let mut next_palette = prev_palette.clone();
                let (mut offset, mut pal_offset, mut prev_pal_offset) = (0, 0, 0);
                while offset < pal_colors_buffer.len() && pal_offset < 256 {
                    let flag_byte = pal_colors_buffer[offset];
                    offset += 1;
                    if (flag_byte & 0x80) != 0 {
                        let increment = ((flag_byte & 0x7F) + 1) as usize;
                        pal_offset += increment;
                    } else if (flag_byte & 0xC0) == 0x40 {
                        prev_pal_offset = pal_colors_buffer[offset] as usize;
                        offset += 1;
                        let increment = ((flag_byte & 0x3F) + 1) as usize;
                        for i in 0..increment {
                            next_palette[pal_offset + i] = prev_palette[prev_pal_offset + i]
                        }
                        pal_offset += increment;
                        prev_pal_offset += increment;
                    } else {
                        let r = PALETTE_MAP_TABLE[(flag_byte & 0x3F) as usize];
                        let flag_byte = pal_colors_buffer[offset];
                        offset += 1;
                        let g = PALETTE_MAP_TABLE[(flag_byte & 0x3F) as usize];
                        let flag_byte = pal_colors_buffer[offset];
                        offset += 1;
                        let b = PALETTE_MAP_TABLE[(flag_byte & 0x3F) as usize];

                        next_palette[pal_offset] =
                            0xFF_000000 + r as u32 + g as u32 * 0x100 + b as u32 * 0x10000;
                        pal_offset += 1;
                    }
                }
                self.smacker_decode_context.palette = next_palette;
            } else {
                stream.seek(SeekFrom::Current(palette_size as i64))?;
            }
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_1) {
            self.unpack_audio(&mut stream, frame_id, frame, 0)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_2) {
            self.unpack_audio(&mut stream, frame_id, frame, 1)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_3) {
            self.unpack_audio(&mut stream, frame_id, frame, 2)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_4) {
            self.unpack_audio(&mut stream, frame_id, frame, 3)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_5) {
            self.unpack_audio(&mut stream, frame_id, frame, 4)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_6) {
            self.unpack_audio(&mut stream, frame_id, frame, 5)?;
        }
        if frame.frame_feature_flags.contains(flags::FrameFeature::HAS_AUDIO_7) {
            self.unpack_audio(&mut stream, frame_id, frame, 6)?;
        }
        if unpack_video {
            self.unpack_video(&mut stream, frame_id, frame)?;
        }
        Ok(())
    }

    fn unpack_audio(
        &mut self,
        stream: &mut Cursor<&[u8]>,
        frame_id: usize,
        frame: &SmackerFrame,
        track_number: usize
    ) -> std::io::Result<()> {
        if !frame.audio_flags[track_number].contains(flags::Audio::PRESENT) {
            return Ok(())
        }
        unimplemented!()
    }

    fn unpack_video(
        &mut self,
        stream: &mut Cursor<&[u8]>,
        frame_id: usize,
        frame: &SmackerFrame
    ) -> std::io::Result<()> {
        unimplemented!()
    }
}