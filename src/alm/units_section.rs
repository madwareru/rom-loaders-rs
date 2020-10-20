use std::io::{Result, Read};
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};

#[derive(Clone, Default, Debug)]
pub struct UnitEntry {
    pub x_coord: u32,
    pub y_coord: u32,
    pub type_id: u16,
    pub face_ide: u16,
    pub special_flags: u32,
    pub server_id: u32,
    pub fraction_id: u32,
    pub sack_id: u32,
    pub view_angle: u32,
    _nops: [f64; 4],
    pub unit_id: u16,
    pub group_id: u32
}
impl Reflectable for UnitEntry {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.x_coord)?;
        reflector.reflect_u32(&mut self.y_coord)?;
        reflector.reflect_u16(&mut self.type_id)?;
        reflector.reflect_u16(&mut self.face_ide)?;
        reflector.reflect_u32(&mut self.special_flags)?;
        reflector.reflect_u32(&mut self.server_id)?;
        reflector.reflect_u32(&mut self.fraction_id)?;
        reflector.reflect_u32(&mut self.sack_id)?;
        reflector.reflect_u32(&mut self.view_angle)?;
        for i in 0..self._nops.len() {
            reflector.reflect_f64(&mut self._nops[i])?;
        }
        reflector.reflect_u16(&mut self.unit_id)?;
        reflector.reflect_u32(&mut self.group_id)
    }
}

#[derive(Debug)]
pub struct UnitsSection {
    pub units: Vec<UnitEntry>
}
impl UnitsSection {
    pub(crate) fn read<TStream: Read>(
        stream: &mut TStream,
        map_info: &super::GeneralMapInfoSection
    ) -> std::io::Result<Self> {
        let size = map_info.unit_count as usize;
        let mut units = Vec::with_capacity(size);
        for _ in 0..size {
            let next_entry = UnitEntry::deserialize(stream, Endianness::LittleEndian)?;
            units.push(next_entry);
        }
        Ok(Self {
            units
        })
    }
}