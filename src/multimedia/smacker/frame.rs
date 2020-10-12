use super::*;
use bitflags::_core::ops::Range;

#[derive(Clone, Debug)]
pub struct SmackerFrameInfo {
    pub frame_range: Range<usize>,
    pub frame_flags: flags::Frame,
    pub frame_feature_flags: flags::FrameFeature,
}