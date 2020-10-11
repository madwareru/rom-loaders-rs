use {
    super::{
        huffman::HeaderTree,
        header::SmackerFileHeader,
        decode_context::SmackerDecodeContext,
        frame::SmackerFrame,
        bit_reader::with_bit_reader,
        flags::*
    },
    crate::shared_types::{U32Wrapper, U8Wrapper},
    bin_serialization_rs::{Reflectable, Endianness},
    std::{
        io::{Read, Seek, Cursor, SeekFrom},
        cmp::Ordering
    }
};

pub struct SmackerFile {
    pub width: u32,
    pub height: u32,
    pub frame_interval: i32,
    pub m_map_tree: Option<HeaderTree>,
    pub m_clr_tree: Option<HeaderTree>,
    pub full_tree: Option<HeaderTree>,
    pub type_tree: Option<HeaderTree>,
    pub smacker_decode_context: SmackerDecodeContext,
    pub frames: Vec<SmackerFrame>
}
impl SmackerFile {
    pub fn load(stream: &mut Cursor<&[u8]>) -> std::io::Result<Self> {
        let header = SmackerFileHeader::deserialize(stream, Endianness::LittleEndian)?;
        let frame_interval = match header.frame_rate.cmp(&0) {
            Ordering::Less => header.frame_rate,
            Ordering::Equal => -header.frame_rate / 100,
            Ordering::Greater => 100
        };
        let mut audio_flags = [0u32, 7];
        let mut audio_rate = header.audio_rate.clone();
        for i in 0..7 {
            audio_flags[i] = audio_rate[i] & 0xFC_000000;
            audio_rate[i] &= 0x00_FFFFFF;
        }
        let num_frames = header.num_frames as usize;
        let mut frame_sizes = Vec::with_capacity(num_frames);
        let mut frame_flags = Vec::with_capacity(num_frames);
        for _ in 0..num_frames {
            let size = U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
            frame_sizes.push(size.0 & 0xFFFFFFF_C);
            frame_flags.push((size.0 & 0x0000000_3) as u8);
        }
        let mut frame_type_flags = Vec::with_capacity(num_frames);
        for _ in 0..num_frames {
            let frame_type_flag_entry = U8Wrapper::deserialize(stream, Endianness::LittleEndian)?;
            frame_type_flags.push(frame_type_flag_entry.0);
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
            frames.push(SmackerFrame {
                frame_bytes
            })
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
            frames
        })
    }
}