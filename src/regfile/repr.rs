use std::io::{Cursor, Read, Seek, SeekFrom};
use super::enumerations::*;
use bin_serialization_rs::{Reflectable, SerializationReflector, Endianness};
use std::collections::{HashMap, VecDeque};
use crate::shared_types::U32Wrapper;
use std::rc::Rc;

#[derive(Clone, Default)]
struct RootRegistryHeader {
    root_offset: u32,
    root_size: u32,
    _registry_flags: u32, // not used anywhere
    registry_eat_size: u32,
    _junk: u32 // not used anywhere
}
impl Reflectable for RootRegistryHeader {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.root_offset)?;
        reflector.reflect_u32(&mut self.root_size)?;
        reflector.reflect_u32(&mut self._registry_flags)?;
        reflector.reflect_u32(&mut self.registry_eat_size)?;
        reflector.reflect_u32(&mut self._junk)
    }
}
impl RootRegistryHeader {
    fn get_data_origin(&self) -> usize {
        0x1C + 0x20 * self.registry_eat_size as usize
    }
}

#[derive(Clone, Default)]
struct RegistryNodeRepresentationTriplet {
    data_byte_0: u32,
    data_byte_1: u32,
    tag: u32,
}
impl Reflectable for RegistryNodeRepresentationTriplet {
    fn reflect<TSerializationReflector: SerializationReflector>(
        &mut self,
        reflector: &mut TSerializationReflector
    ) -> std::io::Result<()> {
        reflector.reflect_u32(&mut self.data_byte_0)?;
        reflector.reflect_u32(&mut self.data_byte_1)?;
        reflector.reflect_u32(&mut self.tag)
    }
}
impl RegistryNodeRepresentationTriplet {
    fn turn_to_node_data(self, root_header: &RootRegistryHeader) -> NodeData {
        match NodeKind::from(self.tag) {
            NodeKind::Directory => {
                let start = root_header.get_data_origin() + self.data_byte_0 as usize * 0x20;
                NodeData::Directory(start, self.data_byte_1 as usize)
            },
            NodeKind::Int => {
                let data = unsafe {
                    let x  = &[self.data_byte_0, self.data_byte_1] as *const u32;
                    let x = x as *const i32;
                    *x
                };
                NodeData::Int(data)
            }
            NodeKind::Float => {
                let data = unsafe {
                    let x  = &[self.data_byte_0, self.data_byte_1] as *const u32;
                    let x = x as *const f64;
                    *x
                };
                NodeData::Float(data)
            }
            NodeKind::String => {
                let start = root_header.get_data_origin() + self.data_byte_0 as usize * 0x20;
                NodeData::String(start, self.data_byte_1 as usize)
            }
            NodeKind::IntArray => {
                let start = root_header.get_data_origin() + self.data_byte_0 as usize * 0x20;
                NodeData::IntArray(start, self.data_byte_1 as usize)
            }
        }
    }
}

struct RegistryHeader {
    node_data: NodeData,
    name: String
}
impl RegistryHeader {
    fn read<TStream: Read + Seek>(
        stream: &mut TStream,
        root: &RootRegistryHeader
    ) -> std::io::Result<Self> {
        stream.seek(SeekFrom::Current(4))?;
        let triplet = RegistryNodeRepresentationTriplet::deserialize(
            stream, 
            Endianness::LittleEndian
        )?;
        let mut char_data = [0u8; 0x10];
        stream.read(&mut char_data)?;
        let name = cp866_rs::decode_bytes(&char_data);
        Ok(Self {
            node_data: triplet.turn_to_node_data(root),
            name
        })
    }
}


pub struct RegistryInfoEnumeration<'a>
{
    pub ints: Vec<&'a str>,
    pub floats: Vec<&'a str>,
    pub strings: Vec<&'a str>,
    pub int_arrays: Vec<&'a str>
}

pub struct Registry {
    stream: Cursor<Vec<u8>>,
    strings_lookup: HashMap<String, ((usize, usize), Option<String>)>,
    int_array_lookup: HashMap<String, ((usize, usize), Option<Vec<i32>>)>,
    ints_lookup: HashMap<String, i32>,
    floats_lookup: HashMap<String, f64>
}
impl Registry {
    pub fn read_from_bytes(bytes: &[u8]) -> Result<Self, RegistryError> {
        let mut data = Vec::new();
        data.extend_from_slice(bytes);
        let mut stream = Cursor::new(data);
        let signature = *U32Wrapper::deserialize(&mut stream, Endianness::LittleEndian)?;
        if signature != 0x31_41_59_26 {
            return Err(RegistryError::IncorrectSignature)
        }
        let root_header = RootRegistryHeader::deserialize(&mut stream, Endianness::LittleEndian)?;
        let mut root_offset = 0x18 + 0x20 * root_header.root_offset as usize;

        let mut queue = VecDeque::new();
        let root_path = Rc::new("".to_string());
        for _ in 0..root_header.root_size {
            queue.push_back((root_path.clone(), root_offset));
            root_offset += 0x20;
        }

        let mut strings_lookup = HashMap::new();
        let mut int_array_lookup = HashMap::new();
        let mut ints_lookup = HashMap::new();
        let mut floats_lookup = HashMap::new();

        while !queue.is_empty() {
            let (parent_path, offset) = queue.pop_front().unwrap();
            let mut new_path = (*parent_path).clone();
            let offset = offset as u64;
            stream.seek(SeekFrom::Start(offset))?;
            let child_node = RegistryHeader::read(&mut stream, &root_header)?;
            match child_node {
                RegistryHeader { node_data, name } => {
                    new_path.push_str(&name);
                    match node_data {
                        NodeData::Directory(offset, count) => {
                            root_offset = offset;
                            for _ in 0..count {
                                queue.push_back((root_path.clone(), root_offset));
                                root_offset += 0x20;
                            }
                        }
                        NodeData::Int(value) => {
                            ints_lookup.insert(new_path, value);
                        }
                        NodeData::Float(value) => {
                            floats_lookup.insert(new_path, value);
                        }
                        NodeData::String(value_offset, length) => {
                            strings_lookup.insert(new_path, ((value_offset, length), None));
                        }
                        NodeData::IntArray(value_offset, length) => {
                            int_array_lookup.insert(new_path, ((value_offset, length), None));
                        }
                    }
                }
            }
        }
        Ok(Self {
            stream,
            strings_lookup,
            int_array_lookup,
            ints_lookup,
            floats_lookup
        })
    }
    pub fn get_int(&self, path: &str) -> Result<i32, RegistryError> {
        match self.ints_lookup.get(path) {
            None => Err(RegistryError::NonExistentIntValue),
            Some(&value) => Ok(value)
        }
    }
    pub fn get_float(&self, path: &str) -> Result<f64, RegistryError> {
        match self.floats_lookup.get(path) {
            None => Err(RegistryError::NonExistentFloatValue),
            Some(&value) => Ok(value)
        }
    }
    fn ensure_string_existence(&mut self, path: &str) -> Result<(), RegistryError> {
        match self.strings_lookup.get_mut(path) {
            None => Err(RegistryError::NonExistentStringValue),
            Some(string_entry) => {
                if let None = string_entry.1 {
                    let (offset, size) = string_entry.0;
                    self.stream.seek(SeekFrom::Start(offset as u64))?;
                    let mut vec = vec![0u8; size];
                    self.stream.read(&mut vec)?;
                    let string = cp866_rs::decode_bytes(&vec);
                    string_entry.1 = Some(string);
                }
                Ok(())
            }
        }
    }
    fn ensure_int_array_existence(&mut self, path: &str) -> Result<(), RegistryError> {
        match self.int_array_lookup.get_mut(path) {
            None => Err(RegistryError::NonExistentIntArrayValue),
            Some(array_entry) => {
                if let None = array_entry.1 {
                    let (offset, size) = array_entry.0;
                    self.stream.seek(SeekFrom::Start(offset as u64))?;
                    let mut vec = Vec::with_capacity(size);
                    for _ in 0..size {
                        let value = *U32Wrapper::deserialize(&mut self.stream, Endianness::LittleEndian)?;
                        let value = unsafe {
                            let x  = &[value] as *const u32;
                            let x = x as *const i32;
                            *x
                        };
                        vec.push(value);
                    }
                    array_entry.1 = Some(vec);
                }
                Ok(())
            }
        }
    }
    pub fn get_int_slice(&mut self, path: &str) -> Result<&[i32], RegistryError> {
        self.ensure_int_array_existence(path)?;
        match self.int_array_lookup.get(path) {
            Some(((_, _), Some(value))) => Ok(value),
            _ => unreachable!()
        }
    }
    pub fn get_string(&mut self, path: &str) -> Result<&str, RegistryError> {
        self.ensure_string_existence(path)?;
        match self.strings_lookup.get(path) {
            Some(((_, _), Some(value))) => Ok(value),
            _ => unreachable!()
        }
    }
    pub fn list_all(&self) -> RegistryInfoEnumeration {
        let mut enumeration = RegistryInfoEnumeration {
            ints: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            int_arrays: Vec::new()
        };
        for (key, _) in self.ints_lookup.iter() { enumeration.ints.push(key); };
        for (key, _) in self.floats_lookup.iter() { enumeration.floats.push(key); };
        for (key, _) in self.strings_lookup.iter() { enumeration.strings.push(key); };
        for (key, _) in self.int_array_lookup.iter() { enumeration.int_arrays.push(key); };
        enumeration.ints.sort();
        enumeration.floats.sort();
        enumeration.int_arrays.sort();
        enumeration.strings.sort();
        enumeration
    }
}