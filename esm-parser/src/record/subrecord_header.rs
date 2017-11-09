use std::io::{ Read, Seek };

use parse::{ Parseable, ParseableWithSize };
use parse_error::ParseError;

use basic_type::{ Long32, Text };

pub struct SubrecordHeader {
    name: Text,
    size: Long32,
}

impl SubrecordHeader {

    pub fn get_name(&self) -> &Text {
        &self.name
    }

    pub fn get_size(&self) -> &Long32 {
        &self.size
    }
}

impl Parseable<SubrecordHeader> for SubrecordHeader {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<SubrecordHeader, ParseError> {
        let name = Text::parse(reader, 4)?;
        let size = Long32::parse(reader)?;
        trace!("subrecord header: name = {}, size = {}", name.0, size.0);
        let header = SubrecordHeader {
            name: name,
            size: size,
        };
        Ok(header)
    }
}
