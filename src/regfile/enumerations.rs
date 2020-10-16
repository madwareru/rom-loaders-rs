#[derive(Debug)]
pub enum RegistryError {
    IncorrectSignature,
    NonExistentIntValue,
    NonExistentFloatValue,
    NonExistentStringValue,
    NonExistentIntArrayValue,
    Io(std::io::Error)
}
impl std::convert::From<std::io::Error> for RegistryError {
    fn from(e: std::io::Error) -> Self {
        RegistryError::Io(e)
    }
}

pub(crate) enum NodeKind {
    Directory,
    Int,
    Float,
    String,
    IntArray
}
impl From<u32> for NodeKind {
    fn from(tag: u32) -> Self {
        match tag {
            0 => NodeKind::String,
            1 => NodeKind::Directory,
            2 => NodeKind::Int,
            4 => NodeKind::Float,
            6 => NodeKind::IntArray,
            _ => unreachable!()
        }
    }
}

pub(crate) enum NodeData {
    Directory(usize, usize),
    Int(i32),
    Float(f64),
    String(usize, usize),
    IntArray(usize, usize),
}