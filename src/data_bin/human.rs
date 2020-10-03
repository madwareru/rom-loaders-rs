use crate::data_bin::section::{SectionDefinition};
use std::io::{Seek, Read, SeekFrom};
use crate::shared_types::CP866String;
use regex::Regex;
use crate::stream_utils::look_ahead;
use bin_serialization_rs::{Reflectable, Endianness, SerializationReflector};

#[derive(PartialEq, Default, Clone, Debug)]
pub struct HumanInfo {
    pub name: CP866String,
    pub details: HumanRecord,
    pub items_wearing: Vec<CP866String>,
}
impl HumanInfo {
    fn read_from_stream<Stream: Seek + Read>(
        stream: &mut Stream,
        human_unit_name_regexp: &Regex
    ) -> Self {
        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        let name_string = CP866String::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        stream.seek(SeekFrom::Current(2)).unwrap();

        let human_rec = HumanRecord::deserialize(
            stream,
            Endianness::LittleEndian,
        ).unwrap();

        let mut items_wearing = Vec::with_capacity(10);
        'item_loop: for _ in 0..10 {
            let look_ahead_v = look_ahead(stream);
            if look_ahead_v >= 128 || look_ahead_v == 0 {
                stream.seek(SeekFrom::Current(1)).unwrap();
                continue;
            }
            let textual_info = CP866String::deserialize(
                stream,
                Endianness::LittleEndian,
            ).unwrap();

            if human_unit_name_regexp.is_match(&textual_info) {
                stream.seek(SeekFrom::Current(-(textual_info.len() as i64 + 1))).unwrap();
                break;
            }
            items_wearing.push(textual_info);

            for __ in 0..3 {
                if look_ahead(stream) != 0 {
                    continue 'item_loop;
                }
                stream.seek(SeekFrom::Current(1)).unwrap();
            }

            break;
        }

        Self {
            name: name_string,
            details: human_rec,
            items_wearing,
        }
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct HumanRecord {
    pub body: i32,
    pub reaction: i32,
    pub mind: i32,
    pub spirit: i32,
    pub health_max: i32,
    pub mana_max: i32,
    pub speed: i32,
    pub rotation_speed: i32,
    pub scan_range: i32,
    pub defence: i32,
    pub skill_general: i32,
    pub skill_blade_fire: i32,
    pub skill_axe_water: i32,
    pub skill_bludgeon_air: i32,
    pub skill_pike_earth: i32,
    pub skill_shooting_astral: i32,
    pub type_id: i32,
    pub face: i32,
    pub gender: i32,
    pub attack_charge_time: i32,
    pub attack_relax_time: i32,
    pub token_size: i32,
    pub movement_type: i32,
    pub dying_time: i32,
    pub server_id: i32,
    pub known_spells: i32,
}
impl Reflectable for HumanRecord {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i32(&mut self.body)?;
        reflector.reflect_i32(&mut self.reaction)?;
        reflector.reflect_i32(&mut self.mind)?;
        reflector.reflect_i32(&mut self.spirit)?;
        reflector.reflect_i32(&mut self.health_max)?;
        reflector.reflect_i32(&mut self.mana_max)?;
        reflector.reflect_i32(&mut self.speed)?;
        reflector.reflect_i32(&mut self.rotation_speed)?;
        reflector.reflect_i32(&mut self.scan_range)?;
        reflector.reflect_i32(&mut self.defence)?;
        reflector.reflect_i32(&mut self.skill_general)?;
        reflector.reflect_i32(&mut self.skill_blade_fire)?;
        reflector.reflect_i32(&mut self.skill_axe_water)?;
        reflector.reflect_i32(&mut self.skill_bludgeon_air)?;
        reflector.reflect_i32(&mut self.skill_pike_earth)?;
        reflector.reflect_i32(&mut self.skill_shooting_astral)?;
        reflector.reflect_i32(&mut self.type_id)?;
        reflector.reflect_i32(&mut self.face)?;
        reflector.reflect_i32(&mut self.gender)?;
        reflector.reflect_i32(&mut self.attack_charge_time)?;
        reflector.reflect_i32(&mut self.attack_relax_time)?;
        reflector.reflect_i32(&mut self.token_size)?;
        reflector.reflect_i32(&mut self.movement_type)?;
        reflector.reflect_i32(&mut self.dying_time)?;
        reflector.reflect_i32(&mut self.server_id)?;
        reflector.reflect_i32(&mut self.known_spells)
    }
}

#[derive(Clone, Debug)]
pub struct HumanSection {
    pub data: Vec<HumanInfo>
}

impl SectionDefinition for HumanSection {
    const HEADER_SIZE: i64 = 0x14F;

    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self {
        let human_unit_name_regexp =
            Regex::new(r"^(?:PC|NPC|NPC\d{1,3}|.|M\d{1,3}|Man.*)_.*")
                .unwrap();

        let entry_count = {
            stream.seek(SeekFrom::Current(4)).unwrap();
            0xD2
        };
        let mut data = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            data.push(HumanInfo::read_from_stream(
                stream,
                &human_unit_name_regexp
            ));
        }
        while look_ahead(stream) == 0 {
            stream.seek(SeekFrom::Current(1)).unwrap();
        }

        Self {
            data
        }
    }
}