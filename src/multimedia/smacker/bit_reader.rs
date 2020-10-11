use std::io::{Read, Error, Cursor};

struct InternalByteReader<'a, TStream : Read> {
    stream: &'a mut TStream
}
impl<'a, TStream : Read> InternalByteReader<'a, TStream> {
    fn from_stream(stream: &'a mut TStream) -> Self {
        Self {
            stream
        }
    }
    fn read_byte(&mut self) -> std::io::Result<u8> {
        let mut b = &mut [0u8];
        let size_read = self.read(b)?;
        Ok( if size_read == 1 { b[0] } else { 0 } )
    }
}

impl<TStream : Read> Read for InternalByteReader<'_, TStream> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

pub struct BitReader<'a, TStream : Read> {
    byte_reader_owned: InternalByteReader<'a, TStream>,
    sub_bit_position : usize,
    last_byte: u8
}
impl<'a, TStream : Read> BitReader<'a, TStream> {
    fn from_stream(stream: &'a mut TStream) -> Self {
        Self {
            byte_reader_owned: InternalByteReader::from_stream(stream),
            sub_bit_position: 0,
            last_byte: 0
        }
    }
    pub fn read_bits(&mut self, count: usize) -> std::io::Result<usize> {
        let mut output = 0;
        for wrote_bits in 0..count {
            if self.sub_bit_position == 0 {
                self.last_byte = self.byte_reader_owned.read_byte()?;
            }
            output |= ((self.last_byte & 0x1) as usize) << wrote_bits as usize;
            self.last_byte >>= 1;
            self.sub_bit_position = (self.sub_bit_position + 1) % 8;
        }
        Ok(output)
    }
}

pub fn with_bit_reader<F, TStream: Read>(stream: &mut TStream, exec_action: F) -> std::io::Result<()>
    where F: FnMut(&mut BitReader<TStream>) -> std::io::Result<()> {
    let mut reader = BitReader::from_stream(stream);
    exec_action(&mut reader)
}