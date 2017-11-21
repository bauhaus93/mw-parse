use std::io::{ Read, Cursor };
use std::fmt;

use parse::Parseable;
use parse_error::ParseError;
use subrecord::{ read_subrecord, read_optional_subrecord };
use subrecord::ambient::Ambient;
use point::Point2D;
use tuple::{ Triple };

pub struct Cell {
    name: String,
    flags: u32,
    grid_pos: Point2D<i32>
}

impl Parseable for Cell {

    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut data: Vec<u8> = Vec::new();
        reader.read_to_end(&mut data)?;
        let mut cursor = Cursor::new(data);

        let name = read_subrecord("NAME", &mut cursor)?;
        let Triple(flags, grid_x, grid_y): Triple<i32> = read_subrecord("DATA", &mut cursor)?;


        let region_name: Option<String> = match flags & 1 {
            0 => read_optional_subrecord("RGNN", &mut cursor)?,
            _ => None
        };

        let nam0: Option<String> = read_optional_subrecord("NAM0", &mut cursor)?;

        let map_color: Option<u32> = match flags & 1 {
            0 => read_optional_subrecord("NAM5",&mut cursor)?,
            _ => None
        };

        let (water_level, ambient): (Option<f32>, Option<Ambient>) = match flags & 1 {
            1 => {
                (Some(read_subrecord("WHGT", &mut cursor)?),
                 Some(read_subrecord("AMBI", &mut cursor)?))
            }
            _ => (None, None)
        };

        match ambient {
            Some(amb) => info!("{}", amb),
            None => {}
        }

        Ok(
            Cell {
                name: name,
                flags: flags as u32,
                grid_pos: Point2D::<i32>::new(grid_x, grid_y)
            }
        )
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CELL: name = {}, flags = {}, grid pos = {}",
            self.name,
            self.flags,
            self.grid_pos
        )
    }
}
