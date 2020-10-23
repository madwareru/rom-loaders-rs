use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use std::io::{Result, Read};
use crate::shared_types::{U32Wrapper, U16Wrapper, U64Wrapper};

#[derive(Clone, Default, Debug)]
pub struct EffectModifier {
    pub modifier_type: u16, // Parameter type in data bin
    pub modifier_value: u32
}
impl Reflectable for EffectModifier {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u16(&mut self.modifier_type)?;
        reflector.reflect_u32(&mut self.modifier_value)
    }
}

#[derive(Clone, Debug)]
pub struct EffectEntry {
    pub corrupt_effect_id: u32,
    pub trap_x: u32,
    pub trap_y: u32,
    pub flags_or_magic_sphere: u16,
    pub service_data: u64,
    pub modifiers: Vec<EffectModifier>
}
impl EffectEntry {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let corrupt_effect_id = *(U32Wrapper::deserialize(stream, endianness))?;
        let trap_x = *(U32Wrapper::deserialize(stream, endianness))?;
        let trap_y = *(U32Wrapper::deserialize(stream, endianness))?;
        let flags_or_magic_sphere = *(U16Wrapper::deserialize(stream, endianness))?;
        let service_data = *(U64Wrapper::deserialize(stream, endianness))?;
        let modifier_count = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut modifiers = Vec::with_capacity(modifier_count as usize);
        for _ in 0..modifier_count {
            modifiers.push(EffectModifier::deserialize(stream, endianness)?);
        }
        Ok(Self {
            corrupt_effect_id,
            trap_x,
            trap_y,
            flags_or_magic_sphere,
            service_data,
            modifiers
        })
    }
}

#[derive(Debug)]
pub struct EffectsSection {
    pub effects: Vec<EffectEntry>
}
impl EffectsSection {
    pub fn read_from_stream<TStream: Read>(stream: &mut TStream, endianness: Endianness) -> Result<Self> {
        let effect_count = *(U32Wrapper::deserialize(stream, endianness))?;
        let mut effects = Vec::with_capacity(effect_count as usize);
        for _ in 0..effect_count {
            effects.push(EffectEntry::read_from_stream(stream, endianness)?);
        }
        Ok(Self { effects })
    }
}