use point::Point2D;
use record::cell::Cell;
use record::cell::interior::InteriorCell;
use record::cell::exterior::ExteriorCell;
use record::cell::common::Common;
use subrecord::ambient::Ambient;

pub struct Builder {
    interior: bool,
    name: String,
    flags: u32,
    region_name: Option<String>,
    grid_pos: Point2D<i32>,
    map_color: Option<u32>,
    water_level: Option<f32>,
    ambient: Option<Ambient>
}

impl Builder {

    pub fn new() -> Builder {
        Builder {
            interior: false,
            name: String::new(),
            region_name: None,
            flags: 0,
            grid_pos: Point2D::<i32>::new(0, 0),
            map_color: None,
            water_level: None,
            ambient: None
        }
    }

    pub fn interior(&mut self) {
        self.interior = true;
    }

    pub fn exterior(&mut self) {
        self.interior = false;
    }

    pub fn name(&mut self, name: String) {
        self.name = name;
    }

    pub fn region_name(&mut self, region_name: String) {
        self.region_name = Some(region_name);
    }

    pub fn flags(&mut self, flags: u32) {
        self.flags = flags;
    }

    pub fn grid_pos(&mut self, grid_pos: Point2D<i32>) {
        self.grid_pos = grid_pos;
    }

    pub fn map_color(&mut self, map_color: u32) {
        self.map_color = Some(map_color);
    }

    pub fn water_level(&mut self, water_level: f32) {
        self.water_level = Some(water_level);
    }

    pub fn ambient(&mut self, ambient: Ambient) {
        self.ambient = Some(ambient);
    }

    pub fn finalize(self) -> Cell {

        let name = if self.name.len() == 0 {
            match self.region_name {
                Some(ref rn) => rn.clone(),
                None => String::new()
            }
        }
        else {
            self.name
        };

        let common_data = Common {
            name: name,
            flags: self.flags,
            grid_pos: self.grid_pos,
        };

        match self.interior {
            true => Cell::Interior(finalize_interior(common_data, self.water_level, self.ambient)),
            false => Cell::Exterior(finalize_exterior(common_data, self.region_name, self.map_color))
        }
    }
}

fn finalize_interior(common_data: Common, water_level: Option<f32>, ambient: Option<Ambient>) -> InteriorCell {
    let water_level = match water_level {
                        Some(lvl) => lvl,
                        None => 0.0
                    };
    let ambient = match ambient {
        Some(amb) => amb,
        None => Ambient::default()
    };
    InteriorCell::new(common_data, water_level, ambient)
}

fn finalize_exterior(common_data: Common, region_name: Option<String>, map_color: Option<u32>) -> ExteriorCell {
    let map_color = match map_color {
        Some(c) => c,
        None => 0
    };
    let region_name = match region_name {
        Some(name) => name,
        None => String::new()
    };
    ExteriorCell::new(common_data, region_name, map_color)
}
