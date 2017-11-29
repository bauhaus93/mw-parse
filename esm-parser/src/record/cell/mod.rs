pub mod exterior;
pub mod interior;
mod builder;
mod common;

use std::io::{ Read, Cursor };
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;
use subrecord::{ read_subrecord, read_optional_subrecord };
use point::Point2D;
use tuple::{ Triple };
use record::cell::builder::Builder;
use record::cell::interior::InteriorCell;
use record::cell::exterior::ExteriorCell;

pub enum Cell {
    Exterior(ExteriorCell),
    Interior(InteriorCell)
}

impl Parseable for Cell {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;
        let record_len = data.len();
        let mut cursor = Cursor::new(data);

        let mut cell_builder = Builder::new();

        cell_builder.name(read_subrecord("NAME", &mut cursor)?);

        let Triple(flags, grid_x, grid_y): Triple<i32> = read_subrecord("DATA", &mut cursor)?;

        cell_builder.flags(flags as u32);
        cell_builder.grid_pos(Point2D::<i32>::new(grid_x, grid_y));

        if cursor.position() == record_len as u64 { //if interior with no data
            return Ok(cell_builder.finalize());
        }

        match flags & 1 {
            0 => {
                cell_builder.exterior();
                match read_optional_subrecord("RGNN", &mut cursor)? {
                    Some(region_name) => cell_builder.region_name(region_name),
                    None => {}
                }
            },
            _ => {
                cell_builder.interior();
            }
        }

        if cursor.position() == record_len as u64 { //if exterior with no data
            return Ok(cell_builder.finalize());
        }

        let _nam0: Option<i32> = read_optional_subrecord("NAM0", &mut cursor)?;  //unused

        match flags & 1 { //interior?
            0 => {
                match read_optional_subrecord("NAM5", &mut cursor)? {
                    Some(sub) => cell_builder.map_color(sub),
                    None => {}
                }
            },
            1 => {
                cell_builder.water_level(read_subrecord("WHGT", &mut cursor)?);
                cell_builder.ambient(read_subrecord("AMBI", &mut cursor)?)
            },
            _ => unreachable!()
        }

        Ok(cell_builder.finalize())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Cell::Interior(ref cell) => write!(f, "{}", cell),
            Cell::Exterior(ref cell) => write!(f, "{}", cell)
        }
    }
}
