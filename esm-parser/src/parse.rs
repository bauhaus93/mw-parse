use std::io::{ Read, Seek };
use parse_error::ParseError;

pub trait Parseable
where Self: Sized {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError>;
}
