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
            stream.seek(SeekFrom::Current(-10)).unwrap();

            match &header_buffer[3..8] {
                [b'S', b'h', b'a', b'p', b'e'] => {
                    stream.seek(SeekFrom::Current(ShapeSection::HEADER_SIZE));
                    shape_section = Some(ShapeSection::read(stream))
                }
                [b'P', b'a', b'r', b'a', b'm'] => {
                    stream.seek(SeekFrom::Current(ParameterSection::HEADER_SIZE));
                    parameter_section = Some(ParameterSection::read(stream))
                }
                [b'I', b't', b'e', b'm', _   ] => {
                    stream.seek(SeekFrom::Current(ItemSection::HEADER_SIZE));
                    item_section = Some(ItemSection::read(stream))
                }
                [b'M', b'a', b'g', b'i', b'c'] => {
                    stream.seek(SeekFrom::Current(MagicItemSection::HEADER_SIZE));
                    magic_item_section = Some(MagicItemSection::read(stream))
                }
                [b'U', b'n', b'i', b't', _   ] => {
                    stream.seek(SeekFrom::Current(UnitSection::HEADER_SIZE));
                    unit_section = Some(UnitSection::read(stream))
                }
                [b'H', b'u', b'm', b'a', b'n'] => {
                    stream.seek(SeekFrom::Current(HumanSection::HEADER_SIZE));
                    human_section = Some(HumanSection::read(stream))
                }
                [b'B', b'u', b'i', b'l', b'd'] => {
                    stream.seek(SeekFrom::Current(StructureSection::HEADER_SIZE));
                    structure_section = Some(StructureSection::read(stream))
                }
                [b'S', b'p', b'e', b'l', b'l'] => {
                    stream.seek(SeekFrom::Current(SpellSection::HEADER_SIZE));
                    spell_section = Some(SpellSection::read(stream))
                }
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