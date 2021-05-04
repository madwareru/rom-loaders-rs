use std::ops::Range;
use std::io::{Result, Read, Seek, Cursor, SeekFrom};
use bin_serialization_rs::{Reflectable, Endianness};
use crate::shared_types::U32Wrapper;

#[derive(Clone)]
pub struct ImageFrameData {
    pub width: u32,
    pub height: u32,
    pub data_range: Range<usize>
}

#[derive(PartialEq, Clone, Copy)]
pub enum ImageType {
    Dot256,
    Dot16,
    Dot16a
}

pub struct ImageData {
    pub image_type: ImageType,
    pub raw: Vec<u8>,
    pub frames: Vec<ImageFrameData>
}

pub(crate) struct SpriteInfo {
    given_sprite_count: u32,
    has_palette: bool
}

pub fn read_image(
    stream: &mut Cursor<&[u8]>,
    image_type: ImageType
) -> Result<ImageData> {
    let SpriteInfo { given_sprite_count, has_palette } = read_sprite_count_info(stream)?;
    match image_type {
        ImageType::Dot16a | ImageType::Dot256 => {
            if has_palette {
                stream.seek(SeekFrom::Current(4 * 256))?;
            }
            read_image_frames(stream, given_sprite_count, image_type)
        },
        ImageType::Dot16 => read_image_frames(stream, given_sprite_count, image_type)
    }
}

pub fn read_palette(
    stream: &mut Cursor<&[u8]>,
    image_type: ImageType
) -> Result<Option<Vec<u32>>> {
    let SpriteInfo { has_palette, .. } = read_sprite_count_info(stream)?;
    match image_type {
        ImageType::Dot16a | ImageType::Dot256 => {
            if !has_palette {
                Ok(None)
            } else {
                let mut v = Vec::with_capacity(256);
                for _ in 0..256 {
                    v.push(*U32Wrapper::deserialize(stream, Endianness::LittleEndian)?);
                }
                Ok(Some(v))
            }
        },
        ImageType::Dot16 => Ok(None)
    }
}

pub(crate) fn read_image_frames(
    stream: &mut Cursor<&[u8]>,
    given_sprite_count: u32,
    image_type: ImageType
) -> Result<ImageData> {
    let mut raw_data_buffer = vec![0u8; 0x10000];
    let mut sprite_data = Vec::new();
    let mut image_frames = Vec::with_capacity(given_sprite_count as usize);
    let mut staging_frame = ImageFrameData {
        width: 0,
        height: 0,
        data_range: 0..0
    };
    for _ in 0..given_sprite_count {
        staging_frame.width = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
        staging_frame.height = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
        let data_size = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)? as usize;
        sprite_data.reserve_exact(data_size);
        if raw_data_buffer.len() < data_size {
            raw_data_buffer.reserve(data_size - raw_data_buffer.len());
        }
        stream.read(&mut raw_data_buffer[0..data_size])?;
        staging_frame.data_range.start = staging_frame.data_range.end;
        staging_frame.data_range.end = staging_frame.data_range.start + data_size;
        (&raw_data_buffer[0..data_size]).iter().for_each(|s| sprite_data.push(*s));
        image_frames.push(staging_frame.clone());
    }
    Ok(ImageData {
        raw: sprite_data,
        frames: image_frames,
        image_type
    })
}

pub(crate) fn read_sprite_count_info(stream: &mut Cursor<&[u8]>) -> Result<SpriteInfo> {
    let old_position = stream.position();
    stream.seek(SeekFrom::End(-4))?;
    let sprite_count = *U32Wrapper::deserialize(stream, Endianness::LittleEndian)?;
    stream.seek(SeekFrom::Start(old_position))?;
    Ok(SpriteInfo{
        given_sprite_count: sprite_count & 0x7FFFFFFF,
        has_palette: sprite_count & 0x80000000 != 0
    })
}