#[derive(Debug)]
pub struct PositionData {
    position: Position,
    rotation: Rotation
}

#[derive(Debug)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32
}

#[derive(Debug)]
pub struct Rotation {
    x: f32,
    y: f32,
    z: f32
}

impl PositionData {

    pub fn new(pos: Position, rot: Rotation) -> PositionData {
        PositionData {
            position: pos,
            rotation: rot
        }
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Position {
        Position {
            x: x,
            y: y,
            z: z
        }
    }
}

impl Rotation {
    pub fn new(x: f32, y: f32, z: f32) -> Rotation {
        Rotation {
            x: x,
            y: y,
            z: z
        }
    }
}
