use std::io::{ Read, Seek };

use subrecord::Subrecord;
use subrecord::subrecord_header::SubrecordHeader;
use parse_error::ParseError;
use parse::{ Parseable, ParseableExact };

impl Subrecord for i32 {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<i32, ParseError> {
        if header.get_size() != 4 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 4, header.get_size()))
        }
        i32::parse(reader)
    }
}

impl Subrecord for i64 {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<i64, ParseError> {
        if header.get_size() != 8 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 8, header.get_size()))
        }
        i64::parse(reader)
    }
}

impl Subrecord for f32 {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<f32, ParseError> {
        if header.get_size() != 4 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 4, header.get_size()))
        }
        f32::parse(reader)
    }
}

impl Subrecord for String {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<String, ParseError> {
        String::parse_exact(reader, header.get_size() as usize)
    }
}
