use std::io::Result;
use bin_serialization_rs::{Reflectable, SerializationReflector};

#[derive(Clone, Default, Debug)]
pub struct GeneralMapInfoSection {
    pub width: u32,
    pub height: u32,
    pub negative_sun_angle: f32,
    pub time_in_minutes: u32,
    pub darkness: u32,
    pub contrast: u32,
    pub use_tiles: u32,
    pub fraction_count: u32,
    pub structure_count: u32,
    pub unit_count: u32,
    pub logic_count: u32,
    pub sack_count: u32,
}

impl Reflectable for GeneralMapInfoSection {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self, reflector: &mut TSerializationReflector
    ) -> Result<()> {
        reflector.reflect_u32(&mut self.width)?;
        reflector.reflect_u32(&mut self.height)?;
        reflector.reflect_f32(&mut self.negative_sun_angle)?;
        reflector.reflect_u32(&mut self.time_in_minutes)?;
        reflector.reflect_u32(&mut self.darkness)?;
        reflector.reflect_u32(&mut self.contrast)?;
        reflector.reflect_u32(&mut self.use_tiles)?;
        reflector.reflect_u32(&mut self.fraction_count)?;
        reflector.reflect_u32(&mut self.structure_count)?;
        reflector.reflect_u32(&mut self.unit_count)?;
        reflector.reflect_u32(&mut self.logic_count)?;
        reflector.reflect_u32(&mut self.sack_count)
    }
}
