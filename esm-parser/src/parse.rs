use std::io::{ Read, Seek };
use parse_error::ParseError;

pub trait Parseable<T> {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<T, ParseError>;
}

pub trait ParseableWithSize<T> {
    fn parse<R: Read + Seek>(reader: &mut R, size: usize) -> Result<T, ParseError>;
}
