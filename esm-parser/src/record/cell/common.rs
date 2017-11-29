use std::fmt;

use point::Point2D;

pub struct Common {
    pub name: String,
    pub flags: u32,
    pub grid_pos: Point2D<i32>
}

impl fmt::Display for Common {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name = {}, flags = {}, {}",
            self.name,
            self.flags,
            self.grid_pos)
    }
}
