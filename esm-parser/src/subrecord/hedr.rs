use std::io::Read;
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;

pub struct Hedr {
    version: f32,
    file_type: u32,
    company_name: String,
    file_description: String,
    num_records: u32
}

impl Hedr {
    pub fn get_num_records(&self) -> u32 {
        self.num_records
    }
}

impl Parseable for Hedr {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(
            Hedr {
                version: f32::from_stream(reader)?,
                file_type: u32::from_stream(reader)?,
                company_name: String::from_stream(&mut reader.by_ref().take(32))?,
                file_description: String::from_stream(&mut reader.by_ref().take(256))?,
                num_records: u32::from_stream(reader)?
            }
        )
    }
}

impl fmt::Display for Hedr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEDR: version = {}, file type = {}, company name = {}, file description = {}, num records = {}",
            self.version,
            self.file_type,
            self.company_name,
            self.file_description,
            self.num_records)
    }
}
