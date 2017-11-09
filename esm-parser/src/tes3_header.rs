use std::str;
use std::fmt;
use std::io::{ Read, Seek };

use parse_error::ParseError;
use parse::{ Parseable, ParseableWithSize };
use basic_type::{ Long32, Float32, Text };
use record::subrecord_header::SubrecordHeader;

pub struct TES3Header {
    version: f32,
    file_type: u32,
    company_name: String,
    description: String,
    num_records: u32
}

impl TES3Header {

    pub fn get_num_records(&self) -> u32 {
        self.num_records
    }
}

impl Parseable<TES3Header> for TES3Header {

    fn parse<R: Read + Seek>(reader: &mut R)  -> Result<TES3Header, ParseError> {
        let sub_header = SubrecordHeader::parse(reader)?;
        if sub_header.get_name().0 != "HEDR" {
            return Err(ParseError::InvalidSubrecordName("HEDR".to_owned(), sub_header.get_name().0.to_owned()));
        }

        Ok(TES3Header {
            version: Float32::parse(reader)?.0,
            file_type: Long32::parse(reader)?.0 as u32,
            company_name: Text::parse(reader, 32)?.0,
            description: Text::parse(reader, 256)?.0,
            num_records: Long32::parse(reader)?.0 as u32,
        })
    }
}

impl fmt::Display for TES3Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tes3 header: version = {}, file_type = {}, company name = \"{}\", description = \"{}\", num records: {}",
               self.version,
               self.file_type,
               self.company_name,
               self.description,
               self.num_records)
    }
}
