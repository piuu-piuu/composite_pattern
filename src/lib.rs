// Element struct.
// #derive i&s needed for Shape.points to function
// otherwise 'traits not implemented for Box...' error
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

//Composite struct
pub struct Shape {
    pub points: Vec<Point>,
}

// Interface common for both element(s) and the composite structure
pub trait Geometry {
    fn transpose(&mut self, dx: u32, dy: u32);
    fn skew(self: Self, x: i16);
}

impl Geometry for Point {
    fn transpose(&mut self, dx: u32, dy: u32) {
        // println!("Point {},{} is transposed...", self.x, self.y);
        self.x += dx;
        self.y += dy;
    }

    fn skew(self, _x: i16) {
        ();
    }
}

// The composite implementation of the interface passes call to all elements
// Static version
impl Geometry for Shape {
    fn transpose(&mut self, dx: u32, dy: u32) {
        println!("{:?}", self.points);
        for point in &mut self.points {
            point.transpose(dx, dy);
        }
        println!("{:?}", self.points);
    }

    fn skew(self, _x: i16) {
        for point in self.points {
            point.skew(_x);
        }
    }
}
