use std::io::Read;

#[derive(Debug)]
pub struct MapObjectsSection {
    pub heights: Vec<u8>
}
impl MapObjectsSection {
    pub(crate) fn read<TStream: Read>(
        stream: &mut TStream,
        map_info: &super::GeneralMapInfoSection
    ) -> std::io::Result<Self> {
        let size = map_info.width as usize * map_info.height as usize;
        let mut heights = vec![0u8; size];
        stream.read(&mut heights)?;
        Ok(Self { heights })
    }
}