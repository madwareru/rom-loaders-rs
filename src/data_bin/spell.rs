use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read};
use crate::shared_types::CP866String;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::stream_utils::read_corrected_entry_count;

#[derive(Clone, Debug)]
pub struct SpellSection {
    pub data: Vec<SpellInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct SpellInfo {
    pub name: CP866String,
    nop: u16,
    pub details: SpellRecord,
    pub textual_info: CP866String,
}
impl Reflectable for SpellInfo {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_composite(&mut self.name)?;
        reflector.reflect_u16(&mut self.nop)?;
        reflector.reflect_composite(&mut self.details)?;
        reflector.reflect_composite(&mut self.textual_info)
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct SpellRecord {
    pub complication_level: i32,
    pub mana_cost: i32,
    pub sphere: i32,
    pub item: i32,
    pub spell_target: i32,
    pub delivery_system: i32,
    pub max_range: i32,
    pub spell_effect_speed: i32,
    pub distribution_system: i32,
    pub radius: i32,
    pub area_effect_affect: i32,
    pub area_effect_duration: i32,
    pub area_effect_frequency: i32,
    pub apply_on_unit_method: i32,
    pub spell_duration: i32,
    pub spell_frequency: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub defensive: i32,
    pub skill_offset: i32,
    pub scroll_cost: i32,
    pub book_cost: i32,
}
impl Reflectable for SpellRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.complication_level)?;
        reflector.reflect_i32(&mut self.mana_cost)?;
        reflector.reflect_i32(&mut self.sphere)?;
        reflector.reflect_i32(&mut self.item)?;
        reflector.reflect_i32(&mut self.spell_target)?;
        reflector.reflect_i32(&mut self.delivery_system)?;
        reflector.reflect_i32(&mut self.max_range)?;
        reflector.reflect_i32(&mut self.spell_effect_speed)?;
        reflector.reflect_i32(&mut self.distribution_system)?;
        reflector.reflect_i32(&mut self.radius)?;
        reflector.reflect_i32(&mut self.area_effect_affect)?;
        reflector.reflect_i32(&mut self.area_effect_duration)?;
        reflector.reflect_i32(&mut self.area_effect_frequency)?;
        reflector.reflect_i32(&mut self.apply_on_unit_method)?;
        reflector.reflect_i32(&mut self.spell_duration)?;
        reflector.reflect_i32(&mut self.spell_frequency)?;
        reflector.reflect_i32(&mut self.damage_min)?;
        reflector.reflect_i32(&mut self.damage_max)?;
        reflector.reflect_i32(&mut self.defensive)?;
        reflector.reflect_i32(&mut self.skill_offset)?;
        reflector.reflect_i32(&mut self.scroll_cost)?;
        reflector.reflect_i32(&mut self.book_cost)
    }
}

impl SectionDefinition for SpellSection {
    const HEADER_SIZE: i64 = 0x14E;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = read_corrected_entry_count(stream) as usize;
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            data.push(
                SpellInfo::deserialize(stream, Endianness::LittleEndian).unwrap()
            );
        }
        Self {
            data
        }
    }
}