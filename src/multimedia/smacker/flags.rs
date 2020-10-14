bitflags! {
    pub struct Header : u8 {
        const HAS_RING_FRAME = 0b001;
        const Y_INTERLACED   = 0b010;
        const Y_DOUBLED      = 0b100;
    }
}
impl Default for Header {
    fn default() -> Self {
        Self::from_bits(0).unwrap()
    }
}

bitflags! {
    pub struct Audio : u32 {
        const COMPRESSED_BINK = 0x0C_000000;
        const IS_STEREO       = 0x10_000000;
        const IS_16_BIT         = 0x20_000000;
        const PRESENT         = 0x40_000000;
        const COMPRESSED      = 0x80_000000;
    }
}
impl Default for Audio {
    fn default() -> Self {
        Self::from_bits(0).unwrap()
    }
}

bitflags! {
    pub struct Frame: u8 {
        const KEYFRAME = 0b01;
        const UNKNOWN  = 0b10;
    }
}
impl Default for Frame {
    fn default() -> Self {
        Self::from_bits(0).unwrap()
    }
}

bitflags! {
    pub struct FrameFeature: u8 {
        const HAS_PALETTE = 0b00000001;
        const HAS_AUDIO_1 = 0b00000010;
        const HAS_AUDIO_2 = 0b00000100;
        const HAS_AUDIO_3 = 0b00001000;
        const HAS_AUDIO_4 = 0b00010000;
        const HAS_AUDIO_5 = 0b00100000;
        const HAS_AUDIO_6 = 0b01000000;
        const HAS_AUDIO_7 = 0b10000000;
    }
}
impl Default for FrameFeature {
    fn default() -> Self {
        Self::from_bits(0).unwrap()
    }
}
