// Element struct
#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

//Static composite struct
// made of elements
pub struct Shape {
    pub points: Vec<Point>,
}

//Dynamic composite struct
pub struct Scene {
    pub shapes: Vec<Box<dyn Geometry>>,
}

// Interface common for both element(s) and the composite structure
pub trait Geometry {
    fn transpose(&mut self, dx: u32, dy: u32);
}

impl Geometry for Point {
    fn transpose(&mut self, dx: u32, dy: u32) {
        self.x += dx;
        self.y += dy;
    }
}

// The composite interface passess calls to elements interfaces with no change.
// Static version,
// for dynamic dispatch see examples/dynamic.rs
impl Geometry for Shape {
    fn transpose(&mut self, dx: u32, dy: u32) {
        println!("{:?}", self.points);
        for point in &mut self.points {
            point.transpose(dx, dy);
        }
        println!("{:?}", self.points);
    }
}
