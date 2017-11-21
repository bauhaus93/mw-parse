use std::fmt;

pub type Position = Point3D<f32>;
pub type Rotation = Point3D<f32>;

pub struct Point2D<T> {
    x: T,
    y: T
}

pub struct Point3D<T> {
    x: T,
    y: T,
    z: T
}

impl<T> Point2D<T>
where T: fmt::Display + Clone {

    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D {
            x: x,
            y: y
        }
    }
}

impl<T> Point3D<T>
where T: fmt::Display + Clone {

    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D {
            x: x,
            y: y,
            z: z
        }
    }
}

impl<T> fmt::Display for Point2D<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.x, self.y)
    }
}

impl<T> fmt::Display for Point3D<T>
where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.x, self.y, self.z)
    }
}
