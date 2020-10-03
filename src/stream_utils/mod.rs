use std::io::{Seek, Read, SeekFrom};
use crate::shared_types::{U8Wrapper, U32Wrapper};
use bin_serialization_rs::{Reflectable, Endianness};

pub fn look_ahead<Stream: Seek + Read>(stream: &mut Stream) -> u8 {
    let v = U8Wrapper::deserialize(stream, Endianness::LittleEndian).unwrap();
    stream.seek(SeekFrom::Current(-1)).unwrap();
    *v
}

pub fn read_entry_count<Stream: Seek + Read>(stream: &mut Stream) -> u32 {
    let cnt = U32Wrapper::deserialize(stream, Endianness::LittleEndian).unwrap();
    *cnt
}

pub fn read_corrected_entry_count<Stream: Seek + Read>(stream: &mut Stream) -> u32 {
    read_entry_count(stream) - 1
}