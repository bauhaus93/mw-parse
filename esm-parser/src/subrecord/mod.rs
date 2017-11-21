
pub mod hedr;
pub mod ambient;
mod header;

use std::io::{ Read, Seek, SeekFrom };

use parse::Parseable;
use parse_error::ParseError;
use subrecord::header::Header;

pub fn read_subrecord<R: Read, T: Parseable>(expected_name: &str, reader: &mut R) -> Result<T, ParseError> {
    let header = Header::from_stream(reader)?;

    if header.get_name() != expected_name {
        return Err(ParseError::InvalidSubrecordName(expected_name.to_owned(), header.get_name().to_owned()));
    }
    Ok(T::from_stream(&mut reader.by_ref().take(header.get_size() as u64))?)
}

pub fn read_optional_subrecord<R: Read + Seek, T: Parseable>(expected_name: &str, reader: &mut R) -> Result<Option<T>, ParseError> {
    let header = Header::from_stream(reader)?;

    match header.get_name() {
        name if name == expected_name => Ok(Some(T::from_stream(&mut reader.by_ref().take(header.get_size() as u64))?)),
        name => {
            reader.seek(SeekFrom::Current(-8)); //already read header name, size fields
            Ok(None)
        }
    }

}
