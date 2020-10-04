mod human;
mod item;
mod magic_item;
mod parameter;
mod shape;
mod spell;
mod structure;
mod unit;
mod section;

use std::io::{Seek, Read, SeekFrom};
use crate::data_bin::section::SectionDefinition;
pub use crate::data_bin::{
    shape::ShapeSection,
    parameter::ParameterSection,
    item::ItemSection,
    magic_item::MagicItemSection,
    unit::UnitSection,
    human::HumanSection,
    structure::StructureSection,
    spell::SpellSection
};

#[derive(Clone, Debug)]
pub struct DataBinContent {
    pub shape_section: Option<ShapeSection>,
    pub item_section: Option<ItemSection>,
    pub magic_item_section: Option<MagicItemSection>,
    pub parameter_section: Option<ParameterSection>,
    pub spell_section: Option<SpellSection>,
    pub structure_section: Option<StructureSection>,
    pub unit_section: Option<UnitSection>,
    pub human_section: Option<HumanSection>
}
impl DataBinContent {
    fn read_section<Stream: Seek + Read, Section: SectionDefinition>(
        stream: &mut Stream
    ) -> Option<Section> {
        stream.seek(SeekFrom::Current(Section::HEADER_SIZE)).unwrap();
        Some(Section::read(stream))
    }

    pub fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self{
        let mut header_buffer = [0u8; 10];
        let mut sections_remain = 8;
        let (
            mut shape_section,
            mut parameter_section,
            mut item_section,
            mut magic_item_section,
            mut human_section,
            mut unit_section,
            mut spell_section,
            mut structure_section
        ) = (None, None, None, None, None, None, None, None);
        while sections_remain > 0 {
            stream.read(&mut header_buffer).unwrap();
            stream.seek(SeekFrom::Current(-(header_buffer.len() as i64))).unwrap();
            match &header_buffer[3..8] {
                [b'S', b'h', b'a', b'p', b'e'] => shape_section = Self::read_section(stream),
                [b'P', b'a', b'r', b'a', b'm'] => parameter_section = Self::read_section(stream),
                [b'I', b't', b'e', b'm', _   ] => item_section = Self::read_section(stream),
                [b'M', b'a', b'g', b'i', b'c'] => magic_item_section = Self::read_section(stream),
                [b'U', b'n', b'i', b't', _   ] => unit_section = Self::read_section(stream),
                [b'H', b'u', b'm', b'a', b'n'] => human_section = Self::read_section(stream),
                [b'B', b'u', b'i', b'l', b'd'] => structure_section = Self::read_section(stream),
                [b'S', b'p', b'e', b'l', b'l'] => spell_section = Self::read_section(stream),
                _ => unreachable!()
            };
            sections_remain -= 1;
        }
        DataBinContent {
            shape_section,
            item_section,
            magic_item_section,
            parameter_section,
            spell_section,
            structure_section,
            unit_section,
            human_section
        }
    }
}