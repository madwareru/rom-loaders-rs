use std::io::Read;
use crate::shared_types::U16Wrapper;
use bin_serialization_rs::{Endianness, Reflectable};

#[derive(Copy, Clone, Debug)]
pub struct TileEntry(u16);
impl TileEntry {
    pub fn is_passable(self) -> bool {
        ((self.0 / 0x100) & 0x20) != 0
    }
    pub fn get_terrain_id(self) -> u8 {
        ((self.0 / 0x100) & 0x03) as u8
    }
    pub fn get_tile_column_id(self) -> u8 {
        ((self.0 & 0xF0) / 0x10) as u8
    }
    pub fn get_tile_row_id(self) -> u8 {
        let terrain_id = self.get_terrain_id();
        ((self.0 & 0xF) as u8).min(if terrain_id != 2 { 13 } else { 7 })
    }
}

#[derive(Debug)]
pub struct TilesSection {
    pub tiles: Vec<TileEntry>
}
impl TilesSection {
    pub(crate) fn read<TStream: Read>(
        stream: &mut TStream,
        map_info: &super::GeneralMapInfoSection
    ) -> std::io::Result<Self> {
        let size = map_info.width as usize * map_info.height as usize;
        let mut tiles = Vec::with_capacity(size);
        for _ in 0..size {
            let next_entry = TileEntry(*U16Wrapper::deserialize(stream, Endianness::LittleEndian)?);
            tiles.push(next_entry);
        }
        Ok(Self {
            tiles
        })
    }
}