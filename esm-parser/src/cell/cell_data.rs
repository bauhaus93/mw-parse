use std::io::{ Read, Seek, SeekFrom };
use std::str;
use std::fmt;

use parse_error::ParseError;
use parse::{ Parseable, ParseableWithSize };
use record::subrecord_header::SubrecordHeader;
use basic_type::{ Long32, Text };
use point::Point2D;

//TODO distinct classes/enum for interior and exterior cells
pub struct CellData {
    name: String,
    flags: i32,
    grid_pos: Point2D<i32>,
    region_name: Option<String>,
}

fn get_exterior_fields<R: Read + Seek>(reader: &mut R) -> Result<(String), ParseError> {
    let sub_header = SubrecordHeader::parse(reader)?;
    let region_name = match sub_header.get_name().0 {
        ref name if name == "RGNN" => Text::parse(reader, sub_header.get_size().0 as usize)?,
        ref unexpected_name => return Err(ParseError::InvalidSubrecordName("RGNN".to_owned(), unexpected_name.to_owned()))
    };
    Ok((region_name.0))
}

impl Parseable<CellData> for CellData {
    fn parse<R: Read + Seek>(reader: &mut R) -> Result<CellData, ParseError> {

        let sub_header = SubrecordHeader::parse(reader)?;
        let name = match sub_header.get_name().0 {
            ref name if name == "NAME" => Text::parse(reader, sub_header.get_size().0 as usize)?,
            ref unexpected_name => return Err(ParseError::InvalidSubrecordName("NAME".to_owned(),unexpected_name.to_owned()))
        };

        let sub_header = SubrecordHeader::parse(reader)?;
        let (flags, grid_x, grid_y) = match sub_header.get_name().0 {
            ref name if name == "DATA" => {
                assert!(sub_header.get_size().0 == 12);
                (Long32::parse(reader)?, Long32::parse(reader)?, Long32::parse(reader)?)
            }
            ref unexpected_name => return Err(ParseError::InvalidSubrecordName("DATA".to_owned(),unexpected_name.to_owned()))
        };

        let region_name = match flags.0 & 1 {
            1 => None,  //Interior cell
            0 => Some(get_exterior_fields(reader)?),      //Exterior cell
            _ => unreachable!()
        };

        let cell_data = CellData {
            name: name.0,
            flags: flags.0,
            grid_pos: Point2D::new(grid_x.0, grid_y.0),
            region_name: region_name
        };
        Ok(cell_data)
    }
}

impl fmt::Display for CellData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.region_name {
            Some(ref reg_name) => write!(f, "cell data: name = {}, flags = {}, grid_pos = {}, region name = {}", self.name, self.flags, self.grid_pos, reg_name),
            None => write!(f, "cell data: name = {}, flags = {}, grid_pos = {}", self.name, self.flags, self.grid_pos)
        }

    }
}
