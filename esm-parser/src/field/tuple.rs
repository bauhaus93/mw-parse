use std::io::Read;

use parse::Parseable;
use parse_error::ParseError;
use tuple::{ Triple, Sextuple };

impl<T> Parseable for Triple<T>
where T: Parseable {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(Triple(  T::from_stream(reader)?,
                    T::from_stream(reader)?,
                    T::from_stream(reader)?))
    }
}

impl<T> Parseable for Sextuple<T>
where T: Parseable {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(Sextuple(    T::from_stream(reader)?,
                        T::from_stream(reader)?,
                        T::from_stream(reader)?,
                        T::from_stream(reader)?,
                        T::from_stream(reader)?,
                        T::from_stream(reader)?,
                    ))
    }
}
