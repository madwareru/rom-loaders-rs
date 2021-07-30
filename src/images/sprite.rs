use std::io::{Read, Seek, Result};
use crate::images::bmp::RawBmp;

pub enum BmpSprite {
    Paletted{
        width: usize,
        height: usize,
        palette: [u32;256],
        palette_indexes: Vec<u8>
    },
    TrueColor{
        width: usize,
        height: usize,
        colors: Vec<u32>
    },
    NotSupported
}
impl BmpSprite {
    pub fn read_from<TStream: Read + Seek>(stream: &mut TStream) -> Result<Self> {
        let raw_bmp = RawBmp::read_from(stream)?;
        match raw_bmp {
            None => Ok(Self::NotSupported),
            Some(bmp) => {
                let upside_down = bmp.header.height > 0;
                let width = bmp.header.width as usize;
                let height = bmp.header.height.abs() as usize;
                match bmp.header.bi_bit_count {
                    8 => {
                        let mut palette_indexes = vec![0u8; bmp.raw_data.len()];
                        let remainder = width % 4;
                        let scanline_padding = if remainder == 0 { 0 } else { 4 - remainder };
                        let mut d_offset = if upside_down { height * width - width } else { 0 };
                        let slide = width * 2;
                        let mut s_offset = 0;
                        for _ in 0..height {
                            for _ in 0..width {
                                palette_indexes[d_offset] = bmp.raw_data[s_offset];
                                s_offset += 1;
                                d_offset += 1;
                            }
                            s_offset += scanline_padding;
                            if !upside_down {
                                d_offset += width;
                                continue;
                            }
                            if d_offset >= slide { d_offset -= slide; }
                        }
                        let mut palette = bmp.palette.unwrap();
                        for entry in palette.iter_mut() {
                            let mut clr = *entry;
                            let b = clr & 0xFF; clr = clr / 0x100;
                            let g = clr & 0xFF; clr = clr / 0x100;
                            let r = clr & 0xFF;

                            let b = if b <= 127 {
                                b * 2
                            } else {
                                255
                            };

                            let g = if g <= 127 {
                                g * 2
                            } else {
                                255
                            };
                            let g = (g * 900) / 1000;

                            let r = if r <= 127 {
                                r * 2
                            }  else {
                                255
                            };
                            *entry = 0xFF000000 | r * 0x10000 | g * 0x100 + b;
                        }
                        Ok(
                            Self::Paletted {
                                width,
                                height,
                                palette,
                                palette_indexes
                            },
                        )
                    },
                    24 => {
                        let mut colors = vec![0xFF000000u32; bmp.raw_data.len() / 3];
                        let remainder = (width * 3) % 4;
                        let scanline_padding = if remainder == 0 { 0 } else { 4 - remainder };
                        let mut d_offset = if upside_down { height * width - width } else { 0 };
                        let slide = width * 2;
                        let mut s_offset = 0;

                        for _ in 0..height {
                            for _ in 0..width {
                                let b = bmp.raw_data[s_offset];
                                colors[d_offset] |= b as u32;
                                s_offset += 1;

                                let g = bmp.raw_data[s_offset];
                                colors[d_offset] |= g as u32 * 0x100;
                                s_offset += 1;

                                let r = bmp.raw_data[s_offset];
                                colors[d_offset] |= r as u32 * 0x10000;
                                s_offset += 1;
                                d_offset += 1;
                            }
                            s_offset += scanline_padding;
                            if !upside_down {
                                d_offset += width;
                                continue;
                            }
                            if d_offset >= slide { d_offset -= slide; }
                        }
                        Ok(Self::TrueColor {
                            width,
                            height,
                            colors
                        })
                    },
                    _ => Ok(Self::NotSupported)
                }
            }
        }
    }
}