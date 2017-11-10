pub mod subrecord_header;
pub mod hedr;
pub mod ambient_data;
pub mod basic_subrecord;
pub mod triple;

use std::io::{ Read, Seek, SeekFrom };

use parse::Parseable;
use parse_error::ParseError;
use subrecord::subrecord_header::SubrecordHeader;


pub trait Subrecord
where Self: Sized {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<Self, ParseError>;

    fn parse_subrecord<R: Read + Seek>(reader: &mut R, expected_name: &str) -> Result<Self, ParseError> {
        let header = SubrecordHeader::parse(reader)?;
        match header.get_name() {
            ref name if *name == expected_name => Ok(Self::parse_with_header(reader, &header)?),
            ref unexpected_name => Err(ParseError::InvalidSubrecordName(expected_name.to_owned(), unexpected_name.to_string()))
        }
    }

    fn parse_optional_subrecord<R: Read + Seek>(reader: &mut R, expected_name: &str) -> Result<Option<Self>, ParseError> {
        match Self::parse_subrecord(reader, expected_name) {
            Ok(result) => Ok(Some(result)),
            Err(ParseError::InvalidSubrecordName(_, _,)) => {
                reader.seek(SeekFrom::Current(-8))?;
                Ok(None)
            }
            Err(ParseError::Io(_)) => Ok(None), //when eof
            Err(other) => Err(other)
        }
    }
}
