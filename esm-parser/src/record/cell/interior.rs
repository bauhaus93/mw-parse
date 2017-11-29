use std::fmt;

use record::cell::common::Common;
use subrecord::ambient::Ambient;

pub struct InteriorCell {
    data: Common,
    water_level: f32,
    ambient: Ambient
}

impl InteriorCell {

    pub fn new(data: Common, water_level: f32, ambient: Ambient) -> InteriorCell {
        InteriorCell {
            data: data,
            water_level: water_level,
            ambient: ambient
        }
    }
}

impl fmt::Display for InteriorCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CELL: interior, {}, water level = {}, {}", self.data, self.water_level, self.ambient)
    }
}
