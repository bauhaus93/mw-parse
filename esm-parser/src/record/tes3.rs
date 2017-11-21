use std::io::Read;
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;
use subrecord::hedr::Hedr;

pub struct TES3 {
    hedr: Hedr
}


impl Parseable for TES3 {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(
            TES3 {
                hedr: Hedr::from_stream(reader)?
            }
        )
    }
}

impl fmt::Display for TES3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TES3: {}",
            self.hedr)
    }
}
