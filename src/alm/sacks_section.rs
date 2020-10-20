use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use std::io::{Result, Read};
use crate::shared_types::{U32Wrapper, U16Wrapper};

#[derive(Clone, Default, Debug)]
pub struct ItemEntry {
    pub id: u32,
    pub wielded: u16,
    pub effect_id: u32
}
impl Reflectable for ItemEntry {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.id)?;
        reflector.reflect_u16(&mut self.wielded)?;
        reflector.reflect_u32(&mut self.effect_id)
    }
}

#[derive(Clone, Debug)]
pub struct SackEntry {
    pub unit_id: u32,
    pub x_coord: u16,
    pub y_coord: u16,
    pub money: u32,
    pub items: Vec<ItemEntry>
}
impl SackEntry {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let item_count = *(U32Wrapper::deserialize(stream, endianness))?;
        let unit_id = *(U32Wrapper::deserialize(stream, endianness))?;
        let x_coord = *(U16Wrapper::deserialize(stream, endianness))?;
        let y_coord = *(U16Wrapper::deserialize(stream, endianness))?;
        let money = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut items = Vec::with_capacity(item_count as usize);
        for _ in 0..item_count {
            items.push(ItemEntry::deserialize(stream, endianness)?);
        }
        Ok(Self {
            unit_id,
            x_coord,
            y_coord,
            money,
            items
        })
    }
}

#[derive(Debug)]
pub struct SacksSection {
    pub sacks: Vec<SackEntry>
}
impl SacksSection {
    pub(crate) fn read<TStream: Read>(
        stream: &mut TStream,
        map_info: &super::GeneralMapInfoSection
    ) -> std::io::Result<Self> {
        let size = map_info.sack_count as usize;
        let mut sacks = Vec::with_capacity(size);
        for _ in 0..size {
            let next_entry = SackEntry::read_from_stream(stream, Endianness::LittleEndian)?;
            sacks.push(next_entry);
        }
        Ok(Self {
            sacks
        })
    }
}