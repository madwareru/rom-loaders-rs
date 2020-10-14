pub struct SmackerDecodeContext {
    pub palette: [(u8, u8, u8); 256],
    pub image: Vec<u8>
}
impl SmackerDecodeContext {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let size_overall = width as usize * height as usize;
        SmackerDecodeContext {
            palette: [(0u8, 0u8, 0u8); 256],
            image: vec![0u8; size_overall]
        }
    }
}