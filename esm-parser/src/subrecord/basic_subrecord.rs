use std::io::{ Read, Seek };

use subrecord::Subrecord;
use subrecord::subrecord_header::SubrecordHeader;
use parse_error::ParseError;
use field::Field;

impl<T: Field> Subrecord for T {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<T, ParseError> {
        T::parse_field(reader, header.get_size() as usize)
    }
}
