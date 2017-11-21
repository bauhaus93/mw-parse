use std::io::Read;
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;

use subrecord::header::Header;

pub struct Hedr {
    version: f32,
    file_type: u32,
    company_name: String,
    file_description: String,
    num_records: u32
}

impl Parseable for Hedr {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let header = Header::from_stream(reader)?;
        
        if header.get_name() != "HEDR" {
            return Err(ParseError::InvalidSubrecordName("HEDR".to_owned(), header.get_name().to_owned()));
        }
        if header.get_size() != 300 {
            return Err(ParseError::InvalidSubrecordSize("HEDR".to_owned(), 300, header.get_size()));
        }

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
