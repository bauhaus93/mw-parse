use std::io::Read;

use parse::Parseable;
use parse_error::ParseError;

pub struct Header {
    name: String,
    size: u32,
    unknown: u32,
    flags: u32
}

impl Header {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_size(&self) -> u32 {
        self.size
    }
}

impl Parseable for Header {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(
            Header {
                name: String::from_stream(&mut reader.by_ref().take(4))?,
                size: u32::from_stream(reader)?,
                unknown: u32::from_stream(reader)?,
                flags: u32::from_stream(reader)?
            }
        )
    }
}
