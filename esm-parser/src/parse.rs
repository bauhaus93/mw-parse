use std::io::Read;
use parse_error::ParseError;

pub trait Parseable
where Self: Sized {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError>;
}
