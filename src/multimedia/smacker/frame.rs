use super::*;

pub struct SmackerFrame {
    pub frame_bytes: Vec<u8>,
    pub frame_flags: flags::Frame,
    pub frame_feature_flags: flags::FrameFeature,
    pub audio_flags: [flags::Audio; 7],
    pub audio_rate: [u32; 7]
}