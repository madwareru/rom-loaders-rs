use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read};
use crate::shared_types::CP866String;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::stream_utils::read_entry_count;

#[derive(Clone, Debug)]
pub struct ShapeSection {
    pub material_data: Vec<ShapeInfo>,
    pub rarity_data: Vec<ShapeInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct ShapeInfo {
    pub name: CP866String,
    nop0: u64,
    nop1: u64,
    pub details: ShapeRecord,
}
impl Reflectable for ShapeInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u64(&mut self.nop0)?;
        reflector.reflect_u64(&mut self.nop1)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct ShapeRecord {
    pub price: f64,
    pub weight: f64,
    pub damage: f64,
    pub to_hit: f64,
    pub defence: f64,
    pub absorption: f64,
    pub mag_cap_level: f64,
}
impl Reflectable for ShapeRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_f64(&mut self.price)?;
        reflector.reflect_f64(&mut self.weight)?;
        reflector.reflect_f64(&mut self.damage)?;
        reflector.reflect_f64(&mut self.to_hit)?;
        reflector.reflect_f64(&mut self.defence)?;
        reflector.reflect_f64(&mut self.absorption)?;
        reflector.reflect_f64(&mut self.mag_cap_level)
    }
}

impl SectionDefinition for ShapeSection {
    const HEADER_SIZE: i64 = 0x66;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = read_entry_count(stream) as usize;
        let mut rarity_data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            rarity_data.push(
                ShapeInfo::deserialize(
                    stream,
                    Endianness::LittleEndian,
                ).unwrap()
            )
        }
        let entry_count = read_entry_count(stream) as usize;
        let mut material_data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            material_data.push(
                ShapeInfo::deserialize(
                    stream,
                    Endianness::LittleEndian,
                ).unwrap()
            )
        }
        Self {
            rarity_data,
            material_data
        }
    }
}