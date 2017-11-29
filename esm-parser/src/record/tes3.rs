use std::io::Read;
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;
use subrecord::hedr::Hedr;
use subrecord::read_subrecord;

pub struct TES3 {
    hedr: Hedr
}

impl TES3 {

    pub fn get_hedr(&self) -> &Hedr {
        &self.hedr
    }
}

impl Parseable for TES3 {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(
            TES3 {
                hedr: read_subrecord("HEDR", reader)?
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
