use bin_serialization_rs::{Reflectable, SerializationReflector};
use std::ops::Deref;
use bitflags::_core::fmt::{Debug, Formatter};

#[derive(Default, Clone, PartialEq)]
pub struct CP866String(String);
impl Reflectable for CP866String {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_cp866_string(&mut self.0)
    }
}
impl AsRef<str> for CP866String {
    fn as_ref(&self) -> &str {
        &(self.0)
    }
}
impl Deref for CP866String {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}
impl Debug for CP866String {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_str("\"")?;
        f.write_str(self)?;
        f.write_str("\"")
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct U32Wrapper(pub u32);
impl Reflectable for U32Wrapper {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.0)
    }
}
impl AsRef<u32> for U32Wrapper {
    fn as_ref(&self) -> &u32 {
        &(self.0)
    }
}
impl Deref for U32Wrapper {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct U16Wrapper(pub u16);
impl Reflectable for U16Wrapper {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_u16(&mut self.0)
    }
}
impl AsRef<u16> for U16Wrapper {
    fn as_ref(&self) -> &u16 {
        &(self.0)
    }
}
impl Deref for U16Wrapper {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct I16Wrapper(pub i16);
impl Reflectable for I16Wrapper {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_i16(&mut self.0)
    }
}
impl AsRef<i16> for I16Wrapper {
    fn as_ref(&self) -> &i16 {
        &(self.0)
    }
}
impl Deref for I16Wrapper {
    type Target = i16;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct U8Wrapper(pub u8);
impl Reflectable for U8Wrapper {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector,
    ) -> std::io::Result<()> {
        reflector.reflect_u8(&mut self.0)
    }
}
impl AsRef<u8> for U8Wrapper {
    fn as_ref(&self) -> &u8 {
        &(self.0)
    }
}
impl Deref for U8Wrapper {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &(self.0)
    }
}