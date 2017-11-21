use std::io::{ Read, Seek };
use std::fmt;

use subrecord::Subrecord;
use subrecord::subrecord_header::SubrecordHeader;
use parse_error::ParseError;
use field::Field;

pub struct Hedr {
    version: f32,
    file_type: u32,
    company_name: String,
    description: String,
    num_records: u32
}

impl Hedr {
    pub fn get_version(&self) -> f32 {
        self.version
    }
    pub fn get_file_type(&self) -> u32 {
        self.file_type
    }

    pub fn get_company_name(&self) -> &str {
        &self.company_name
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_num_records(&self) -> u32 {
        self.num_records
    }
}

impl Subrecord for Hedr {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<Hedr, ParseError> {
        if header.get_size() != 300 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 300, header.get_size()))
        }
        Ok(Hedr {
            version: f32::parse_field_fixed(reader)?,
            file_type: i32::parse_field_fixed(reader)? as u32,
            company_name: String::parse_field(reader, 32)?,
            description: String::parse_field(reader, 256)?,
            num_records: i32::parse_field_fixed(reader)? as u32,
        })
    }
}

impl fmt::Display for Hedr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEDR: version = {}, file type = {}, company_name = {}, description = {}, num_records = {}",
            self.version,
            self.file_type,
            self.company_name,
            self.description,
            self.num_records)
    }
}
