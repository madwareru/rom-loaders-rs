pub struct SmackerDecodeContext {
    pub palette: [u32; 256],
    pub image: Vec<u8>
}
impl SmackerDecodeContext {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        let size_overall = width as usize * height as usize;
        SmackerDecodeContext {
            palette: [0u32; 256],
            image: vec![0u8; size_overall]
        }
    }
}