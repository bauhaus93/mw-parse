use std::io::{ Read, Seek };

use parse::{ Parseable, ParseableExact };
use parse_error::ParseError;

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

impl Parseable for SubrecordHeader {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<SubrecordHeader, ParseError> {
        let name = String::parse_exact(reader, 4)?;
        let size = i32::parse(reader)?;
        trace!("subrecord header: name = {}, size = {}", name, size);
        let header = SubrecordHeader {
            name: name,
            size: size as u32,
        };
        Ok(header)
    }
}
