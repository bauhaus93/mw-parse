use std::io::Read;
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;

pub struct Ambient {
    color_ambient: u32,
    color_sunlight: u32,
    color_fog: u32,
    density_fog: f32
}

impl Default for Ambient {

    fn default() -> Self {
        Ambient {
            color_ambient: 0,
            color_sunlight: 0,
            color_fog: 0,
            density_fog: 0.5
        }
    }
}

impl Parseable for Ambient {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        Ok(
            Ambient {
                color_ambient: u32::from_stream(reader)?,
                color_sunlight: u32::from_stream(reader)?,
                color_fog: u32::from_stream(reader)?,
                density_fog: f32::from_stream(reader)?,
            }
        )
    }
}

impl fmt::Display for Ambient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AMBI: ambient color = {}, sunlight color = {}, fog color = {}, fog density = {}",
            self.color_ambient,
            self.color_sunlight,
            self.color_fog,
            self.density_fog,
        )
    }
}
