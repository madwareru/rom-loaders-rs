pub mod header {
    pub const HAS_RING_FRAME: u8 = 0x01;
    pub const Y_INTERLACED:   u8 = 0x02;
    pub const Y_DOUBLED:      u8 = 0x04;
}

pub mod audio {
    pub const COMPRESSED_BINK: u32 = 0x02_000000 | 0x08_000000;
    pub const IS_STEREO:       u32 = 0x10_000000;
    pub const IS16BIT:         u32 = 0x20_000000;
    pub const PRESENT:         u32 = 0x40_000000;
    pub const COMPRESSED:      u32 = 0x80_000000;
}

pub mod frame {
    pub const KEY_FRAME: u8 = 0x01;
    pub const UNKNOWN:   u8 = 0x02;
}

pub mod frame_feature {
    pub const HAS_PALETTE: u8 = 0x01;
    pub const HAS_AUDIO_1: u8 = 0x02;
    pub const HAS_AUDIO_2: u8 = 0x04;
    pub const HAS_AUDIO_3: u8 = 0x08;
    pub const HAS_AUDIO_4: u8 = 0x10;
    pub const HAS_AUDIO_5: u8 = 0x20;
    pub const HAS_AUDIO_6: u8 = 0x40;
    pub const HAS_AUDIO_7: u8 = 0x80;
}