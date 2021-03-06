use std::io::{Result, Read};
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};

#[derive(Clone, Default, Debug)]
pub struct FractionEntry {
    pub color_id: u32,
    pub flags: u32,
    pub money: u32,
    pub name: String,
    pub diplomacy_states: [u16;0x10]
}
impl Reflectable for FractionEntry {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.color_id)?;
        reflector.reflect_u32(&mut self.flags)?;
        reflector.reflect_u32(&mut self.money)?;
        let mut name_bytes = [0u8;0x20];
        for i in 0..name_bytes.len() {
            reflector.reflect_u8(&mut name_bytes[i])?;
        }
        self.name = cp866_rs::decode_bytes(&name_bytes);
        for i in 0..self.diplomacy_states.len() {
            reflector.reflect_u16(&mut self.diplomacy_states[i])?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FractionsSection {
    pub fractions: Vec<FractionEntry>
}
impl FractionsSection {
    pub(crate) fn read<TStream: Read>(
        stream: &mut TStream,
        map_info: &super::GeneralMapInfoSection
    ) -> std::io::Result<Self> {
        let size = map_info.fraction_count as usize;
        let mut fractions = Vec::with_capacity(size);
        for _ in 0..size {
            let next_entry = FractionEntry::deserialize(stream, Endianness::LittleEndian)?;
            fractions.push(next_entry);
        }
        Ok(Self {
            fractions
        })
    }
}