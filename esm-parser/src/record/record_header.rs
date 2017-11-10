use std::io::{ Read, Seek };

use parse::{ Parseable, ParseableExact };
use parse_error::ParseError;

pub struct RecordHeader {
    name: String,
    size: u32,
    unknown: u32,
    flags: u32
}

impl RecordHeader {

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}

impl Parseable for RecordHeader {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<RecordHeader, ParseError> {
        let name = String::parse_exact(reader, 4)?;
        let size = i32::parse(reader)?;
        let unknown = i32::parse(reader)?;
        let flags = i32::parse(reader)?;
        trace!("record header: name = {}, size = {}", name, size);
        let header = RecordHeader {
            name: name,
            size: size as u32,
            unknown: unknown as u32,
            flags: flags as u32
        };
        Ok(header)
    }
}
