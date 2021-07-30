use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use std::io::{Result, Seek, Read, SeekFrom};
use crate::shared_types::U32Wrapper;

#[derive(Default, Debug, Clone)]
pub struct RawBmpHeader {
    pub width: u32,
    pub height: i32,
    _bi_planes: u16,
    pub bi_bit_count: u16,
    _bi_compression: u32,
    _bi_size_image: u32,
    _bi_x_pels_per_meter: u32,
    _bi_y_pels_per_meter: u32,
    _bi_clr_used: u32,
    _bi_clr_important: u32,
}
impl Reflectable for RawBmpHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.width)?;
        reflector.reflect_i32(&mut self.height)?;
        reflector.reflect_u16(&mut self._bi_planes)?;
        reflector.reflect_u16(&mut self.bi_bit_count)?;
        reflector.reflect_u32(&mut self._bi_compression)?;
        reflector.reflect_u32(&mut self._bi_size_image)?;
        reflector.reflect_u32(&mut self._bi_x_pels_per_meter)?;
        reflector.reflect_u32(&mut self._bi_y_pels_per_meter)?;
        reflector.reflect_u32(&mut self._bi_clr_used)?;
        reflector.reflect_u32(&mut self._bi_clr_important)
    }
}

pub struct RawBmp {
    pub header: RawBmpHeader,
    pub palette: Option<[u32; 256]>, // Exists only for 8bit images
    pub scanline_padding: usize,
    pub raw_data: Vec<u8>
}
impl RawBmp {
    pub fn read_from<TStream: Read + Seek>(stream: &mut TStream) -> Result<Option<Self>> {
        let magic = &mut [0u8, 0u8];
        stream.read(magic)?;
        if magic != &[b'B', b'M'] {
            return Ok(None); // not a bmp file. Just return None in this case
        }
        stream.seek(SeekFrom::Current(8))?; // ignoring 8 unused bytes
        let bfh_pixel_data = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)? as u64;
        let bi_version = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
        if bi_version != 40 {
            Ok(None)
        } else {
            let header = RawBmpHeader::deserialize(stream, Endianness::LittleEndian)?;
            let palette = if header.bi_bit_count == 8 {
                let mut arr = [0u32; 256];
                for arr_entry in arr.iter_mut() {
                    *arr_entry = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
                }
                Some(arr)
            } else {
                None
            };
            let scanline_size = header.width as usize * header.bi_bit_count as usize / 8;
            let remainder = scanline_size % 4;
            let scanline_padding = if remainder == 0 { 0 } else { 4 - remainder };
            let data_size = (scanline_size + scanline_padding) * header.height.abs() as usize;
            let mut raw_data = vec![0u8; data_size];
            stream.seek(SeekFrom::Start(bfh_pixel_data))?;
            stream.read(&mut raw_data)?;
            Ok(Some(Self {
                header,
                palette,
                scanline_padding,
                raw_data
            }))
        }
    }
}