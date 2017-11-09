use std::io::{ Read, Seek };

use parse::{ Parseable, ParseableWithSize };
use parse_error::ParseError;

use basic_type::{ Long32, Text };

pub struct RecordHeader {
    name: Text,
    size: Long32,
    unknown: Long32,
    flags: Long32
}

impl RecordHeader {

    pub fn get_name(&self) -> &Text {
        &self.name
    }

    pub fn get_size(&self) -> &Long32 {
        &self.size
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
            name: name,
            size: size,
            unknown: unknown,
            flags: flags
        };
        Ok(header)
    }
}
