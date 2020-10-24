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
                match bmp.header.bi_bit_count {
                    8 => {
                        let width = bmp.header.width as usize;
                        let height = bmp.header.height as usize;
                        let mut palette_indexes = vec![0u8; bmp.raw_data.len()];
                        let mut d_offset = height * width - width;
                        let slide = width * 2;
                        let mut s_offset = 0;
                        for _ in 0..height {
                            for _ in 0..width {
                                palette_indexes[d_offset] = bmp.raw_data[s_offset];
                                s_offset += 1;
                                d_offset += 1;
                            }
                            d_offset -= slide;
                        }
                        let mut palette = bmp.palette.unwrap();
                        for entry in palette.iter_mut() {
                            let mut clr = *entry;
                            let b = clr & 0xFF; clr = clr / 0x100;
                            let g = clr & 0xFF; clr = clr / 0x100;
                            let r = clr & 0xFF;

                            let b = if b < 170 {
                                b * 3 / 2
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
                            *entry = 0xFF000000 + b * 0x10000 + g * 0x100 + r;
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
                        let width = bmp.header.width as usize;
                        let height = bmp.header.height as usize;
                        let mut colors = vec![0xFF000000u32; bmp.raw_data.len() / 3];
                        let mut d_offset = height * width - width;
                        let slide = width * 2;
                        let mut s_offset = 0;
                        for _ in 0..height {
                            for _ in 0..width {
                                let b = bmp.raw_data[s_offset];
                                colors[d_offset] += b as u32 * 0x10000;
                                s_offset += 1;

                                let g = bmp.raw_data[s_offset];
                                colors[d_offset] += g as u32 * 0x100;
                                s_offset += 1;

                                let r = bmp.raw_data[s_offset];
                                colors[d_offset] += r as u32;
                                s_offset += 1;
                                d_offset += 1;
                            }
                            d_offset -= slide;
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