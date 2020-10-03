use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read};
use crate::shared_types::CP866String;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::stream_utils::read_entry_count;

#[derive(Clone, Debug)]
pub struct ParameterSection {
    pub data: Vec<ParameterInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct ParameterInfo {
    pub name: CP866String,
    nop: u16,
    pub details: ParameterRecord,
}
impl Reflectable for ParameterInfo {
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
pub struct ParameterRecord {
    pub cost_mp: i32,
    pub affect_min: i32,
    pub affect_max: i32,
    pub usable_by: i32,
    pub in_weapon: i32,
    pub in_shield: i32,
    pub nop1: i32,
    pub in_ring: i32,
    pub in_amulet: i32,
    pub in_helm: i32,
    pub in_mail: i32,
    pub in_cuirass: i32,
    pub in_bracers: i32,
    pub in_gauntlets: i32,
    pub nop2: i32,
    pub in_boots: i32,
    pub in_weapon2: i32,
    pub nop3: i32,
    pub nop4: i32,
    pub in_ring2: i32,
    pub in_amulet2: i32,
    pub in_hat: i32,
    pub in_robe: i32,
    pub in_cloak: i32,
    pub nop5: i32,
    pub in_gloves: i32,
    pub nop6: i32,
    pub in_shoes: i32,
}
impl Reflectable for ParameterRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.cost_mp)?;
        reflector.reflect_i32(&mut self.affect_min)?;
        reflector.reflect_i32(&mut self.affect_max)?;
        reflector.reflect_i32(&mut self.usable_by)?;
        reflector.reflect_i32(&mut self.in_weapon)?;
        reflector.reflect_i32(&mut self.in_shield)?;
        reflector.reflect_i32(&mut self.nop1)?;
        reflector.reflect_i32(&mut self.in_ring)?;
        reflector.reflect_i32(&mut self.in_amulet)?;
        reflector.reflect_i32(&mut self.in_helm)?;
        reflector.reflect_i32(&mut self.in_mail)?;
        reflector.reflect_i32(&mut self.in_cuirass)?;
        reflector.reflect_i32(&mut self.in_bracers)?;
        reflector.reflect_i32(&mut self.in_gauntlets)?;
        reflector.reflect_i32(&mut self.nop2)?;
        reflector.reflect_i32(&mut self.in_boots)?;
        reflector.reflect_i32(&mut self.in_weapon2)?;
        reflector.reflect_i32(&mut self.nop3)?;
        reflector.reflect_i32(&mut self.nop4)?;
        reflector.reflect_i32(&mut self.in_ring2)?;
        reflector.reflect_i32(&mut self.in_amulet2)?;
        reflector.reflect_i32(&mut self.in_hat)?;
        reflector.reflect_i32(&mut self.in_robe)?;
        reflector.reflect_i32(&mut self.in_cloak)?;
        reflector.reflect_i32(&mut self.nop5)?;
        reflector.reflect_i32(&mut self.in_gloves)?;
        reflector.reflect_i32(&mut self.nop6)?;
        reflector.reflect_i32(&mut self.in_shoes)
    }
}

impl SectionDefinition for ParameterSection {
    const HEADER_SIZE: i64 = 0x123;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = read_entry_count(stream) as usize;
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            data.push(
                ParameterInfo::deserialize(stream, Endianness::LittleEndian).unwrap()
            );
        }
        Self {
            data
        }
    }
}