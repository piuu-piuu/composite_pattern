use rand::Rng;
use std::collections::HashSet;

// Element struct.
// #derive is needed for Shape.points to function
// otherwise 'traits not implemented for Box...' error
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

//Composite struct
struct Shape {
    points: HashSet<Box<Point>>,
}

// Interface common for both element(s) and the composite structure
trait Geometry {
    fn transpose(self, dx: u32, dy: u32);
    fn skew(&self, x: i16);
}

impl Geometry for Point {
    fn transpose(mut self, dx: u32, dy: u32) {
        print!("Point ({},{}) transposed", self.x, self.y);
        self.x += dx;
        self.y += dy;
        println!(" into ({},{}).", self.x, self.y)
    }

    fn skew(&self, x: i16) {
        todo!();
    }
}

// The composite implementation of the interface passes call to all elements
impl Geometry for Shape {
    fn transpose(self, dx: u32, dy: u32) {
        for point in self.points.into_iter() {
            point.transpose(dx, dy)
        }
    }

    fn skew(&self, x: i16) {
        todo!();
    }
}

// dynamic version
// fn dyn_dispatch_geometry(shape: &dyn Geometry, d: i16) {
//     shape.skew(d);
// }

fn main() {
    let mut composite_shape = Shape {
        points: HashSet::new(),
    };

    for _ in 1..10 {
        let boxedpoint = Box::new(Point {
            x: rand::thread_rng().gen_range(1..101),
            y: rand::thread_rng().gen_range(1..101),
        });
        composite_shape.points.insert(boxedpoint);
    }

    // transposing all points as if they are the one structure

    // dynamic dispatch
    // dyn_dispatch_geometry(&composite_shape, rand::thread_rng().gen_range(1..360));

    // static dispatch
    composite_shape.transpose(
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101),
    );
}
