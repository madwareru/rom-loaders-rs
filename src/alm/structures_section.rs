use std::io::{Result, Read};
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};

#[derive(Default, Clone, Debug)]
pub struct BridgeInfo {
    pub width: u32,
    pub height: u32,
}
impl Reflectable for BridgeInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.width)?;
        reflector.reflect_u32(&mut self.height)
    }
}

#[derive(Default, Clone, Debug)]
pub struct StructureEntry {
    pub x_coord: u32,
    pub y_coord: u32,
    pub type_id: u32,
    pub health: u16,
    pub fraction_id: u32,
    pub id: u16,
    pub bridge_info: BridgeInfo
}
impl Reflectable for StructureEntry {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.x_coord)?;
        reflector.reflect_u32(&mut self.y_coord)?;
        reflector.reflect_u32(&mut self.type_id)?;
        reflector.reflect_u16(&mut self.health)?;
        reflector.reflect_u32(&mut self.fraction_id)?;
        reflector.reflect_u16(&mut self.id)?;
        if self.type_id == 33 { // magic number resembling the only variable width/height bridge available in the game
            reflector.reflect_composite(&mut self.bridge_info)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct StructuresSection {
    pub structures: Vec<StructureEntry>
}
impl StructuresSection {
    pub(crate) fn read<TStream: Read>(
        stream: &mut TStream,
        map_info: &super::GeneralMapInfoSection
    ) -> std::io::Result<Self> {
        let size = map_info.structure_count as usize;
        let mut structures = Vec::with_capacity(size);
        for _ in 0..size {
            let next_entry = StructureEntry::deserialize(stream, Endianness::LittleEndian)?;
            structures.push(next_entry);
        }
        Ok(Self {
            structures
        })
    }
}