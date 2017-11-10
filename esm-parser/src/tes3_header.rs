use std::str;
use std::fmt;
use std::io::{ Read, Seek };

use parse_error::ParseError;
use parse::Parseable;
use subrecord::Subrecord;
use subrecord::hedr::Hedr;

pub struct TES3Header {
    hedr: Hedr
}

impl TES3Header {

    pub fn get_hedr(&self) -> &Hedr {
        &self.hedr
    }
}

impl Parseable for TES3Header {

    fn parse<R: Read + Seek>(reader: &mut R)  -> Result<TES3Header, ParseError> {
        let hedr = Hedr::parse_subrecord(reader, "HEDR")?;

        Ok(TES3Header {
            hedr: hedr
        })
    }
}

impl fmt::Display for TES3Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tes3 header: {}", self.hedr)
    }
}
