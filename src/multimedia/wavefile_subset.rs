use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use std::io::{Read, Seek, SeekFrom};
use crate::shared_types::U16Wrapper;

const RIFF: u32 = 0x46_46_49_52;
const WAVE: u32 = 0x45_56_41_57;
const FMT: u32  = 0x20_74_6D_66;
const DATA: u32 = 0x61_74_61_64;
const PCM: u16 = 0x0001;

#[derive(Clone, Default)]
struct RiffHeader {
    signature: u32,
    size: u32,
    wave_id: u32
}
impl Reflectable for RiffHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.signature)?;
        reflector.reflect_u32(&mut self.size)?;
        reflector.reflect_u32(&mut self.wave_id)?;
        assert_eq!(self.signature, RIFF);
        assert_eq!(self.wave_id, WAVE);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct ChunkHeader {
    signature: u32,
    size: u32
}
impl Reflectable for ChunkHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector:
        &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.signature)?;
        reflector.reflect_u32(&mut self.size)
    }
}

#[derive(Clone, Default)]
pub struct FmtChunk {
    header: ChunkHeader,
    pub format: u16,
    pub channels: u16,
    pub sampling_rate: u32,
    pub data_rate: u32,
    pub bytes_per_sample: u16,
    pub bits_per_sample: u16
}
impl Reflectable for FmtChunk {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector:
        &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.header)?;
        reflector.reflect_u16(&mut self.format)?;
        reflector.reflect_u16(&mut self.channels)?;
        reflector.reflect_u32(&mut self.sampling_rate)?;
        reflector.reflect_u32(&mut self.data_rate)?;
        reflector.reflect_u16(&mut self.bytes_per_sample)?;
        reflector.reflect_u16(&mut self.bits_per_sample)?;
        assert_eq!(self.header.signature, FMT);
        assert_eq!(self.header.size, 0x10);
        assert_eq!(self.format, PCM);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct WavContent {
    pub fmt: FmtChunk,
    pub data: Vec<u16>
}
impl WavContent {
    pub fn read<Stream: Read+Seek>(stream: &mut Stream) -> std::io::Result<Self> {
        let _riff_header = RiffHeader::deserialize(stream, Endianness::LittleEndian)?;
        let fmt = FmtChunk::deserialize(stream, Endianness::LittleEndian)?;
        let mut data_header = ChunkHeader::deserialize(stream, Endianness::LittleEndian)?;
        while data_header.signature != DATA {
            stream.seek(SeekFrom::Current(data_header.size as i64))?;
            data_header = ChunkHeader::deserialize(stream, Endianness::LittleEndian)?;
        }
        let data_size = (data_header.size / 2) as usize;
        let mut data = Vec::with_capacity(data_size);
        for _ in 0..data_size {
            let sample = U16Wrapper::deserialize(stream, Endianness::LittleEndian)?;
            data.push(*sample);
        }
        Ok(WavContent {
            fmt,
            data
        })
    }
}