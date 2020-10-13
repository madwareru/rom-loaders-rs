use bin_serialization_rs::{
    Reflectable,
    SerializationReflector
};

#[derive(PartialEq, Default, Clone, Debug)]
pub(crate) struct SmackerFileHeader {
    pub(crate) signature: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) num_frames: u32,
    pub(crate) frame_rate: i32,
    pub(crate) header_flags: u32,
    pub(crate) audio_size: [u32; 7],
    pub(crate) trees_size: u32,
    pub(crate) m_map_size: u32,
    pub(crate) m_clr_size: u32,
    pub(crate) full_size: u32,
    pub(crate) type_size: u32,
    pub(crate) audio_rate: [u32; 7],
    pub(crate) dummy: u32,
}
impl Reflectable for SmackerFileHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.signature)?;
        reflector.reflect_u32(&mut self.width)?;
        reflector.reflect_u32(&mut self.height)?;
        reflector.reflect_u32(&mut self.num_frames)?;
        reflector.reflect_i32(&mut self.frame_rate)?;
        reflector.reflect_u32(&mut self.header_flags)?;
        for i in 0..7 {
            reflector.reflect_u32(&mut self.audio_size[i])?;
        }
        reflector.reflect_u32(&mut self.trees_size)?;
        reflector.reflect_u32(&mut self.m_map_size)?;
        reflector.reflect_u32(&mut self.m_clr_size)?;
        reflector.reflect_u32(&mut self.full_size)?;
        reflector.reflect_u32(&mut self.type_size)?;
        for i in 0..7 {
            reflector.reflect_u32(&mut self.audio_rate[i])?;
        }
        reflector.reflect_u32(&mut self.dummy)
    }
}