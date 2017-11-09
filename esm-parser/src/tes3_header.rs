use std::str;
use std::fmt;
use std::io::{ Read, Seek };

use parse_error::ParseError;
use parse::{ Parseable, ParseableWithSize };
use basic_type::{ Long32, Float32, Text };
use record::subrecord_header::SubrecordHeader;

pub struct TES3Header {
    version: Float32,
    file_type: Long32,
    company_name: Text,
    description: Text,
    num_records: Long32
}

impl TES3Header {

    pub fn get_num_records(&self) -> &Long32 {
        &self.num_records
    }
}

impl Parseable<TES3Header> for TES3Header {

    fn parse<R: Read + Seek>(reader: &mut R)  -> Result<TES3Header, ParseError> {
        let sub_header = SubrecordHeader::parse(reader)?;
        if sub_header.get_name().0 != "HEDR" {
            return Err(ParseError::InvalidSubrecordName("HEDR".to_owned(), sub_header.get_name().0.to_owned()));
        }

        Ok(TES3Header {
            version: Float32::parse(reader)?,
            file_type: Long32::parse(reader)?,
            company_name: Text::parse(reader, 32)?,
            description: Text::parse(reader, 256)?,
            num_records: Long32::parse(reader)?,
        })
    }
}

impl fmt::Display for TES3Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tes3 header: version = {}, file_type = {}, company name = \"{}\", description = \"{}\", num records: {}",
               self.version.0,
               self.file_type.0,
               self.company_name.0,
               self.description.0,
               self.num_records.0)
    }
}
