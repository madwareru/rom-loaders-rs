use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read, SeekFrom};
use crate::shared_types::CP866String;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use crate::stream_utils::look_ahead;

#[derive(Clone, Debug)]
pub struct UnitSection {
    pub data: Vec<UnitInfo>
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct UnitInfo {
    pub name: CP866String,
    pub details: UnitRecord,
    pub textual_info: CP866String
}
impl UnitInfo {
    fn read_from_stream<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        let name = CP866String::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        stream.seek(SeekFrom::Current(2)).unwrap();

        let details = UnitRecord::deserialize(
            stream,
            Endianness::LittleEndian
        ).unwrap();

        let textual_info = CP866String::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        Self {
            name,
            details,
            textual_info
        }
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct UnitRecord {
    pub body: i32,
    pub reaction: i32,
    pub mind: i32,
    pub spirit: i32,
    pub health_max: i32,
    pub hp_regeneration: i32,
    pub mana_max: i32,
    pub mp_regeneration: i32,
    pub speed: i32,
    pub rotation_speed: i32,
    pub scan_range: i32,
    pub physical_min: i32,
    pub physical_max: i32,
    pub attack_kind: i32,
    pub to_hit: i32,
    pub defence: i32,
    pub absorption: i32,
    pub attack_charge_time: i32,
    pub attack_relax_time: i32,
    pub protect_fire: i32,
    pub protect_water: i32,
    pub protect_air: i32,
    pub protect_earth: i32,
    pub protect_astral: i32,
    pub resist_blade: i32,
    pub resist_axe: i32,
    pub resist_bludgeon: i32,
    pub resist_pike: i32,
    pub resist_shooting: i32,
    pub type_id: i32,
    pub face: i32,
    pub token_size: i32,
    pub movement_type: i32,
    pub dying_time: i32,
    pub withdraw: i32,
    pub wimpy: i32,
    pub see_invisible: i32,
    pub xp_value: i32,
    pub treasure1_gold: i32,
    pub treasure_min1: i32,
    pub treasure_max1: i32,
    pub treasure2_item: i32,
    pub treasure_min2: i32,
    pub treasure_max2: i32,
    pub treasure3_magic: i32,
    pub treasure_min3: i32,
    pub treasure_max3: i32,
    pub power: i32,
    pub spell1: i32,
    pub probability1: i32,
    pub spell2: i32,
    pub probability2: i32,
    pub spell3: i32,
    pub probability3: i32,
    pub spell_power: i32
}
impl Reflectable for UnitRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.body)?;
        reflector.reflect_i32(&mut self.reaction)?;
        reflector.reflect_i32(&mut self.mind)?;
        reflector.reflect_i32(&mut self.spirit)?;
        reflector.reflect_i32(&mut self.health_max)?;
        reflector.reflect_i32(&mut self.hp_regeneration)?;
        reflector.reflect_i32(&mut self.mana_max)?;
        reflector.reflect_i32(&mut self.mp_regeneration)?;
        reflector.reflect_i32(&mut self.speed)?;
        reflector.reflect_i32(&mut self.rotation_speed)?;
        reflector.reflect_i32(&mut self.scan_range)?;
        reflector.reflect_i32(&mut self.physical_min)?;
        reflector.reflect_i32(&mut self.physical_max)?;
        reflector.reflect_i32(&mut self.attack_kind)?;
        reflector.reflect_i32(&mut self.to_hit)?;
        reflector.reflect_i32(&mut self.defence)?;
        reflector.reflect_i32(&mut self.absorption)?;
        reflector.reflect_i32(&mut self.attack_charge_time)?;
        reflector.reflect_i32(&mut self.attack_relax_time)?;
        reflector.reflect_i32(&mut self.protect_fire)?;
        reflector.reflect_i32(&mut self.protect_water)?;
        reflector.reflect_i32(&mut self.protect_air)?;
        reflector.reflect_i32(&mut self.protect_earth)?;
        reflector.reflect_i32(&mut self.protect_astral)?;
        reflector.reflect_i32(&mut self.resist_blade)?;
        reflector.reflect_i32(&mut self.resist_axe)?;
        reflector.reflect_i32(&mut self.resist_bludgeon)?;
        reflector.reflect_i32(&mut self.resist_pike)?;
        reflector.reflect_i32(&mut self.resist_shooting)?;
        reflector.reflect_i32(&mut self.type_id)?;
        reflector.reflect_i32(&mut self.face)?;
        reflector.reflect_i32(&mut self.token_size)?;
        reflector.reflect_i32(&mut self.movement_type)?;
        reflector.reflect_i32(&mut self.dying_time)?;
        reflector.reflect_i32(&mut self.withdraw)?;
        reflector.reflect_i32(&mut self.wimpy)?;
        reflector.reflect_i32(&mut self.see_invisible)?;
        reflector.reflect_i32(&mut self.xp_value)?;
        reflector.reflect_i32(&mut self.treasure1_gold)?;
        reflector.reflect_i32(&mut self.treasure_min1)?;
        reflector.reflect_i32(&mut self.treasure_max1)?;
        reflector.reflect_i32(&mut self.treasure2_item)?;
        reflector.reflect_i32(&mut self.treasure_min2)?;
        reflector.reflect_i32(&mut self.treasure_max2)?;
        reflector.reflect_i32(&mut self.treasure3_magic)?;
        reflector.reflect_i32(&mut self.treasure_min3)?;
        reflector.reflect_i32(&mut self.treasure_max3)?;
        reflector.reflect_i32(&mut self.power)?;
        reflector.reflect_i32(&mut self.spell1)?;
        reflector.reflect_i32(&mut self.probability1)?;
        reflector.reflect_i32(&mut self.spell2)?;
        reflector.reflect_i32(&mut self.probability2)?;
        reflector.reflect_i32(&mut self.spell3)?;
        reflector.reflect_i32(&mut self.probability3)?;
        reflector.reflect_i32(&mut self.spell_power)
    }
}

impl SectionDefinition for UnitSection {
    const HEADER_SIZE: i64 = 0x026B;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let entry_count = {
            stream.seek(SeekFrom::Current(4)).unwrap();
            0x38
        };
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            data.push(UnitInfo::read_from_stream(stream));
        }
        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }
        Self {
            data
        }
    }
}