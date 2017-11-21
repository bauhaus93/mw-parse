use std::io::{ Read, Seek };
use std::mem;

use parse::Parseable;
use parse_error::ParseError;
use subrecord::subrecord_header::SubrecordHeader;
use subrecord::Subrecord;
use field::Field;

pub struct Triple<T>(pub T, pub T, pub T);
pub struct Sextuple<T>(pub T, pub T, pub T, pub T, pub T, pub T);

impl<T: Field> Subrecord for Triple<T> {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<Triple<T>, ParseError> {
        if header.get_size() != 3 * mem::size_of::<T>() as u32 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 3 * mem::size_of::<T>() as u32, header.get_size()))
        }
        Ok(Triple(T::parse_field_fixed(reader)?, T::parse_field_fixed(reader)?, T::parse_field_fixed(reader)?))
    }
}


impl<T: Field> Subrecord for Sextuple<T> {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<Sextuple<T>, ParseError> {
        if header.get_size() != 6 * mem::size_of::<T>() as u32 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 6 * mem::size_of::<T>() as u32, header.get_size()))
        }
        Ok(Sextuple(T::parse_field_fixed(reader)?, T::parse_field_fixed(reader)?, T::parse_field_fixed(reader)?,
                    T::parse_field_fixed(reader)?, T::parse_field_fixed(reader)?, T::parse_field_fixed(reader)?))
    }
}
