use std::io::{ Read, Seek };

use parse::Parseable;
use parse_error::ParseError;
use subrecord::subrecord_header::SubrecordHeader;
use subrecord::Subrecord;

pub struct Triple<T>(pub T, pub T, pub T);

impl Subrecord for Triple<i32> {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<Triple<i32>, ParseError> {
        if header.get_size() != 12 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 12, header.get_size()))
        }
        Ok(Triple(i32::parse(reader)?, i32::parse(reader)?, i32::parse(reader)?))
    }
}
