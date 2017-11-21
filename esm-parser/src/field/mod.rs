pub mod basic_type;

use std::io::{ Read, Seek };

use parse_error::ParseError;

pub trait Field
where Self: Sized {
    fn parse_field<R: Read + Seek>(reader: &mut R, size: usize) -> Result<Self, ParseError>;
    fn parse_field_fixed<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError>;
}
