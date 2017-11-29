use std::fmt;

use record::cell::common::Common;

pub struct ExteriorCell {
    data: Common,
    region_name: String,
    map_color: u32
}

impl ExteriorCell {

    pub fn new(data: Common, region_name: String, map_color: u32) -> ExteriorCell {
        ExteriorCell {
            data: data,
            region_name: region_name,
            map_color: map_color
        }
    }
}

impl fmt::Display for ExteriorCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CELL: exterior, {}, region name = {}, map color = {}",
            self.data,
            self.region_name,
            self.map_color
        )
    }
}
