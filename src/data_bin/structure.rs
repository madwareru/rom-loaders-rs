use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read};
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::shared_types::CP866String;
use crate::stream_utils::read_corrected_entry_count;

#[derive(Clone, Debug)]
pub struct StructureSection {
    pub data: Vec<StructureInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct StructureInfo {
    pub name: CP866String,
    nop: u16,
    pub details: StructureRecord,
}
impl Reflectable for StructureInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct StructureRecord {
    pub size_x: i32,
    pub size_y: i32,
    pub scan_range: i32,
    pub health_max: i16,
    pub passability: i8,
    pub building_present: i8,
    pub start_id: i32,
    pub tiles: i16,
    nop: i16
}
impl Reflectable for StructureRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.size_x)?;
        reflector.reflect_i32(&mut self.size_y)?;
        reflector.reflect_i32(&mut self.scan_range)?;
        reflector.reflect_i16(&mut self.health_max)?;
        reflector.reflect_i8(&mut self.passability)?;
        reflector.reflect_i8(&mut self.building_present)?;
        reflector.reflect_i32(&mut self.start_id)?;
        reflector.reflect_i16(&mut self.tiles)?;
        reflector.reflect_i16(&mut self.nop)
    }
}
impl SectionDefinition for StructureSection {
    const HEADER_SIZE: i64 = 0x56;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = read_corrected_entry_count(stream) as usize;
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            data.push(
                StructureInfo::deserialize(stream, Endianness::LittleEndian).unwrap()
            );
        }
        Self {
            data
        }
    }
}