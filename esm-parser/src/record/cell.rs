use std::io::{ Read, Seek };
use std::str;
use std::fmt;

use parse_error::ParseError;
use parse::Parseable;
use point::Point2D;
use subrecord::Subrecord;
use subrecord::triple::Triple;

//TODO distinct classes/enum for interior and exterior cells
pub struct Cell {
    name: String,
    flags: i32,
    grid_pos: Point2D<i32>,
    region_name: Option<String>,
}

impl Parseable for Cell {

    fn parse<R: Read + Seek>(reader: &mut R) -> Result<Cell, ParseError> {
        let name = String::parse_subrecord(reader, "NAME")?;
        let Triple(flags, grid_x, grid_y) = Triple::<i32>::parse_subrecord(reader, "DATA")?;

        let region_name = match flags & 1 {
            0 => Some(String::parse_subrecord(reader, "RGNN")?),
            _ => None
        };

        let cell_data = Cell {
            name: name,
            flags: flags,
            grid_pos: Point2D::new(grid_x, grid_y),
            region_name: region_name
        };
        Ok(cell_data)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.region_name {
            Some(ref reg_name) => write!(f, "cell data: name = {}, flags = {}, grid_pos = {}, region name = {}", self.name, self.flags, self.grid_pos, reg_name),
            None => write!(f, "cell data: name = {}, flags = {}, grid_pos = {}", self.name, self.flags, self.grid_pos)
        }

    }
}
