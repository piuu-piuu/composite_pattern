use std::collections::HashSet;

// Element struct.
// #derive is needed for Shape.points to function
// otherwise 'traits not implemented for Box...' error
#[derive(PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

//Composite struct
pub struct Shape {
    pub points: HashSet<Box<Point>>,
}

// Interface common for both element(s) and the composite structure
pub trait Geometry {
    fn transpose(self, dx: u32, dy: u32);
    fn skew(self: Box<Self>, x: i16);
}

impl Geometry for Point {
    fn transpose(mut self, dx: u32, dy: u32) {
        print!("Point ({},{}) transposed", self.x, self.y);
        self.x += dx;
        self.y += dy;
        println!(" into ({},{}).", self.x, self.y)
    }

    fn skew(self: Box<Self>, _x: i16) {
        ();
    }
}

// The composite implementation of the interface passes call to all elements
// Static version
impl Geometry for Shape {
    fn transpose(self, dx: u32, dy: u32) {
        for point in self.points.into_iter() {
            point.transpose(dx, dy)
        }
    }

    fn skew(self: Box<Self>, _x: i16) {
        for point in self.points {
            point.skew(_x);
        }
    }
}
