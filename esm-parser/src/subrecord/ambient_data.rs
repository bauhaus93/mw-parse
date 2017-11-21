use std::io::{ Read, Seek };
use std::fmt;

use subrecord::Subrecord;
use subrecord::subrecord_header::SubrecordHeader;
use parse_error::ParseError;
use field::Field;

pub struct AmbientData {
    ambient_color: u32,
    sunlight_color: u32,
    fog_color: u32,
    fog_density: f32,
}
impl Subrecord for AmbientData {
    fn parse_with_header<R: Read + Seek>(reader: &mut R, header: &SubrecordHeader) -> Result<AmbientData, ParseError> {
        if header.get_size() != 16 {
            return Err(ParseError::InvalidSubrecordSize(header.get_name().to_owned(), 16, header.get_size()))
        }
        Ok(AmbientData {
            ambient_color: i32::parse_field_fixed(reader)? as u32,
            sunlight_color: i32::parse_field_fixed(reader)? as u32,
            fog_color: i32::parse_field_fixed(reader)? as u32,
            fog_density: f32::parse_field_fixed(reader)?
        })
    }
}

impl fmt::Display for AmbientData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AMBI: ambient color = 0x{:X}, sunlight color = 0x{:X}, fog color = 0x{:X}, fog density = {}",
            self.ambient_color,
            self.sunlight_color,
            self.fog_color,
            self.fog_density)
    }
}
