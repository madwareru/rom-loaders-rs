use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use super::huffman::HeaderTree;
use std::io::{Read, Seek, Cursor, SeekFrom};
use std::cmp::Ordering;
use crate::shared_types::{U32Wrapper, U8Wrapper};
use crate::multimedia::smacker::bit_reader::with_bit_reader;

#[derive(PartialEq, Default, Clone, Debug)]
pub struct SmackerFileHeader {
    pub signature: u32,
    pub width: u32,
    pub height: u32,
    pub num_frames: u32,
    pub frame_rate: i32,
    pub header_flags: u32,
    pub audio_size: [u32; 7],
    pub trees_size: u32,
    pub m_map_size: u32,
    pub m_clr_size: u32,
    pub full_size: u32,
    pub type_size: u32,
    pub audio_rate: [u32; 7],
    pub dummy: u32,
}
impl Reflectable for SmackerFileHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.signature)?;
        reflector.reflect_u32(&mut self.width)?;
        reflector.reflect_u32(&mut self.height)?;
        reflector.reflect_u32(&mut self.num_frames)?;
        reflector.reflect_i32(&mut self.frame_rate)?;
        reflector.reflect_u32(&mut self.header_flags)?;
        for i in 0..7 {
            reflector.reflect_u32(&mut self.audio_size[i])?;
        }
        reflector.reflect_u32(&mut self.trees_size)?;
        reflector.reflect_u32(&mut self.m_map_size)?;
        reflector.reflect_u32(&mut self.m_clr_size)?;
        reflector.reflect_u32(&mut self.full_size)?;
        reflector.reflect_u32(&mut self.type_size)?;
        for i in 0..7 {
            reflector.reflect_u32(&mut self.audio_rate[i])?;
        }
        reflector.reflect_u32(&mut self.dummy)
    }
}

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
    pub fn load<TStream: Read>(stream: &mut Cursor<TStream>) -> std::io::Result<Self> {
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
                m_map_tree = Some(HeaderTree::new(bit_reader, header.m_map_size as usize)?);
            }
            if bit_reader.read_bits(1)? == 1 {
                m_clr_tree = Some(HeaderTree::new(bit_reader, header.m_clr_size as usize)?);
            }
            if bit_reader.read_bits(1)? == 1 {
                full_tree = Some(HeaderTree::new(bit_reader, header.full_size as usize)?);
            }
            if bit_reader.read_bits(1)? == 1 {
                type_tree = Some(HeaderTree::new(bit_reader, header.type_size as usize)?);
            }
            Ok(())
        })?;

        stream.seek(SeekFrom::Start(trees_start_position + header.trees_size as u64))?;

        unimplemented!()
    }
}


pub struct SmackerDecodeContext {

}

pub struct SmackerFrame {

}