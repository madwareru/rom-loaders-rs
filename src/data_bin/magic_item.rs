use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read};
use crate::shared_types::CP866String;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::stream_utils::read_corrected_entry_count;

#[derive(Clone, Debug)]
pub struct MagicItemSection {
    pub data: Vec<MagicItemInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct MagicItemInfo {
    pub name: CP866String,
    nop0: u16,
    pub details: MagicItemRecord,
    nop1: u8,
    pub textual_info: CP866String
}
impl Reflectable for MagicItemInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop0)?;
        reflector.reflect_composite(&mut self.details)?;
        reflector.reflect_u8(&mut self.nop1)?;
        reflector.reflect_composite(&mut self.textual_info)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct MagicItemRecord {
    pub price: i32,
    pub weight: i32,
}
impl Reflectable for MagicItemRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector:
        &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.price)?;
        reflector.reflect_i32(&mut self.weight)
    }
}

impl SectionDefinition for MagicItemSection {
    const HEADER_SIZE: i64 = 0x23;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = read_corrected_entry_count(stream) as usize;
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            data.push(
                MagicItemInfo::deserialize(stream, Endianness::LittleEndian).unwrap()
            );
        }
        Self {
            data
        }
    }
}