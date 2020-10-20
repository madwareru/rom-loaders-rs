mod general_map_info_section;
mod tiles_section;
mod height_map_section;
mod map_objects_section;
mod structures_section;
mod fractions_section;
mod units_section;
mod triggers_section;
mod sacks_section;
mod effects_section;

pub use {
    general_map_info_section::*,
    tiles_section::*,
    height_map_section::*,
    map_objects_section::*,
    structures_section::*,
    fractions_section::*,
    units_section::*,
    triggers_section::*,
    sacks_section::*,
    effects_section::*
};
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use std::convert::TryFrom;
use std::io::{ErrorKind, Read, Cursor, Seek, SeekFrom};

#[derive(Copy, Clone, PartialEq, Debug, num_enum::TryFromPrimitive)]
#[repr(u32)]
pub enum SectionKind {
    General,
    Tiles,
    HeightMap,
    MapObjects,
    Structures,
    Fractions,
    Units,
    Triggers,
    Sacks,
    Effects
}
impl Default for SectionKind {
    fn default() -> Self {
        SectionKind::General
    }
}

#[derive(Debug)]
pub struct AlmMap {
    pub general_info: GeneralMapInfoSection,
    pub tiles: Option<TilesSection>,
    pub height_map: Option<HeightMapSection>,
    pub map_objects: Option<MapObjectsSection>,
    pub structures: Option<StructuresSection>,
    pub fractions: Option<FractionsSection>,
    pub units: Option<UnitsSection>,
    pub triggers: Option<TriggersSection>,
    pub sacks: Option<SacksSection>,
    pub effects: Option<EffectsSection>
}
impl AlmMap {
    pub fn read<TStream: Read + AsRef<[u8]>>(stream: &mut Cursor<TStream>) -> std::io::Result<Self> {
        let alm_header = AlmHeader::deserialize(stream, Endianness::LittleEndian)?;
        let general_map_info_header = SectionHeader::deserialize(stream, Endianness::LittleEndian)?;
        assert_eq!(general_map_info_header.section_kind, SectionKind::General);
        let mut position_before_section_read = stream.position();
        let general_info = GeneralMapInfoSection::deserialize(stream, Endianness::LittleEndian)?;
        stream.seek(SeekFrom::Start(position_before_section_read + general_map_info_header.data_size as u64))?;
        let (
            mut tiles,
            mut height_map,
            mut map_objects,
            mut structures,
            mut fractions,
            mut units,
            mut triggers,
            mut sacks,
            mut effects
        ) = (
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None
        );

        for _ in 1..alm_header.section_count {
            let next_section_header = SectionHeader::deserialize(stream, Endianness::LittleEndian)?;
            position_before_section_read = stream.position();
            match next_section_header.section_kind {
                SectionKind::Tiles => {
                    tiles = Some(TilesSection::read(stream, &general_info)?);
                },
                SectionKind::HeightMap => {
                    height_map = Some(HeightMapSection::read(stream, &general_info)?);
                },
                SectionKind::MapObjects => {
                    map_objects = Some(MapObjectsSection::read(stream, &general_info)?);
                },
                SectionKind::Structures => {
                    structures = Some(StructuresSection::read(stream, &general_info)?);
                },
                SectionKind::Fractions => {
                    fractions = Some(FractionsSection::read(stream, &general_info)?);
                },
                SectionKind::Units => {
                    units = Some(UnitsSection::read(stream, &general_info)?);
                },
                SectionKind::Triggers => {
                    triggers = Some(TriggersSection::read_from_stream(stream, Endianness::LittleEndian)?);
                },
                SectionKind::Sacks => {
                    sacks = Some(SacksSection::read(stream, &general_info)?);
                },
                SectionKind::Effects => {
                    effects = Some(EffectsSection::read_from_stream(stream, Endianness::LittleEndian)?);
                },
                _ => unreachable!()
            }
            stream.seek(SeekFrom::Start(position_before_section_read + next_section_header.data_size as u64))?;
        }
        Ok(Self {
            general_info,
            tiles,
            height_map,
            map_objects,
            structures,
            fractions,
            units,
            triggers,
            sacks,
            effects
        })
    }
}

#[derive(Clone, Default)]
struct AlmHeader {
    signature: u32,
    header_size: u32,
    mysterious_size: u32,
    section_count: u32,
    random_seed: u32
}
impl Reflectable for AlmHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.signature)?;
        reflector.reflect_u32(&mut self.header_size)?;
        reflector.reflect_u32(&mut self.mysterious_size)?;
        reflector.reflect_u32(&mut self.section_count)?;
        reflector.reflect_u32(&mut self.random_seed)
    }
}

#[derive(Clone, Default)]
struct SectionHeader {
    _some_id: u32,
    header_size: u32,
    data_size: u32,
    section_kind: SectionKind,
    random_seed: u32
}
impl Reflectable for SectionHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self._some_id)?;
        reflector.reflect_u32(&mut self.header_size)?;
        reflector.reflect_u32(&mut self.data_size)?;
        let mut sec_kind = self.section_kind as u32;
        reflector.reflect_u32(&mut sec_kind)?;
        self.section_kind =
            SectionKind::try_from(sec_kind)
            .map_err(|_| std::io::Error::from(ErrorKind::InvalidInput))?;
        reflector.reflect_u32(&mut self.random_seed)
    }
}