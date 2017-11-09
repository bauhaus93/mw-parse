use std::io::{ Read, Seek };

use parse::{ Parseable, ParseableWithSize };
use parse_error::ParseError;

use basic_type::{ Long32, Text };

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

impl Parseable<RecordHeader> for RecordHeader {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<RecordHeader, ParseError> {
        let name = Text::parse(reader, 4)?;
        let size = Long32::parse(reader)?;
        let unknown = Long32::parse(reader)?;
        let flags = Long32::parse(reader)?;
        trace!("record header: name = {}, size = {}", name.0, size.0);
        let header = RecordHeader {
            name: name.0,
            size: size.0 as u32,
            unknown: unknown.0 as u32,
            flags: flags.0 as u32
        };
        Ok(header)
    }
}
