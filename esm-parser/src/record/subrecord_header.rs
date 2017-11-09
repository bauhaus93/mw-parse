use std::io::{ Read, Seek };

use parse::{ Parseable, ParseableWithSize };
use parse_error::ParseError;

use basic_type::{ Long32, Text };

pub struct SubrecordHeader {
    name: String,
    size: u32,
}

impl SubrecordHeader {

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}

impl Parseable<SubrecordHeader> for SubrecordHeader {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<SubrecordHeader, ParseError> {
        let name = Text::parse(reader, 4)?;
        let size = Long32::parse(reader)?;
        trace!("subrecord header: name = {}, size = {}", name.0, size.0);
        let header = SubrecordHeader {
            name: name.0,
            size: size.0 as u32,
        };
        Ok(header)
    }
}
