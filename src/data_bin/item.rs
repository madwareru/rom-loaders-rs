use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read};
use crate::shared_types::CP866String;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::stream_utils::read_corrected_entry_count;

#[derive(Clone, Debug)]
pub struct ItemSection {
    pub wieldables: Vec<ItemInfo>,
    pub shields: Vec<ItemInfo>,
    pub weapons: Vec<ItemInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct ItemInfo {
    pub name: CP866String,
    nop: u16,
    pub details: ItemRecord,
}
impl Reflectable for ItemInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector:
        &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct ItemRecord {
    pub shape: i32,
    pub material: i32,
    pub price: i32,
    pub weight: i32,
    pub slot: i32,
    pub attack_type: i32,
    pub physical_min: i32,
    pub physical_max: i32,
    pub to_hit: i32,
    pub defence: i32,
    pub absorption: i32,
    pub range: i32,
    pub charge: i32,
    pub relax: i32,
    pub two_handed: i32,
    pub suitable_for: i32,
    pub other_parameter: i32,
    pub mysterious_field0: i32,
    pub mysterious_field1: i32,
    pub mysterious_field2: i32,
}
impl Reflectable for ItemRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.shape)?;
        reflector.reflect_i32(&mut self.material)?;
        reflector.reflect_i32(&mut self.price)?;
        reflector.reflect_i32(&mut self.weight)?;
        reflector.reflect_i32(&mut self.slot)?;
        reflector.reflect_i32(&mut self.attack_type)?;
        reflector.reflect_i32(&mut self.physical_min)?;
        reflector.reflect_i32(&mut self.physical_max)?;
        reflector.reflect_i32(&mut self.to_hit)?;
        reflector.reflect_i32(&mut self.defence)?;
        reflector.reflect_i32(&mut self.absorption)?;
        reflector.reflect_i32(&mut self.range)?;
        reflector.reflect_i32(&mut self.charge)?;
        reflector.reflect_i32(&mut self.relax)?;
        reflector.reflect_i32(&mut self.two_handed)?;
        reflector.reflect_i32(&mut self.suitable_for)?;
        reflector.reflect_i32(&mut self.other_parameter)?;
        reflector.reflect_i32(&mut self.mysterious_field0)?;
        reflector.reflect_i32(&mut self.mysterious_field1)?;
        reflector.reflect_i32(&mut self.mysterious_field2)
    }
}

impl SectionDefinition for ItemSection {
    const HEADER_SIZE: i64 = 0xAD;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = read_corrected_entry_count(stream) as usize;
        let mut wieldables = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            wieldables.push(
                ItemInfo::deserialize(
                    stream,
                    Endianness::LittleEndian,
                ).unwrap()
            );
        }
        let entry_count = read_corrected_entry_count(stream) as usize;
        let mut shields = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            shields.push(
                ItemInfo::deserialize(
                    stream,
                    Endianness::LittleEndian,
                ).unwrap()
            );
        }
        let entry_count = read_corrected_entry_count(stream) as usize;
        let mut weapons = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            weapons.push(
                ItemInfo::deserialize(
                    stream,
                    Endianness::LittleEndian,
                ).unwrap()
            );
        }

        Self {
            wieldables,
            shields,
            weapons
        }
    }
}