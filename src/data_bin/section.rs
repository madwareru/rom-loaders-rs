use std::io::{Seek, Read};

pub(crate) trait SectionDefinition {
    const HEADER_SIZE: i64;
    fn read<Stream: Seek + Read>(stream: &mut Stream) -> Self;
}