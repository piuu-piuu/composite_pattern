use std::collections::HashSet;

// derive is needed for Shape.points to function
// otherwise 'traits not implemented for Box...' error
#[derive(PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

struct Shape {
    points: HashSet<Box<Point>>,
}

trait Geometry {
    fn transpose(self, dx: u32, dy: u32);
}

trait Parenting {
    fn give_birth(self, child: Point);
    fn good_bye(self, child: Point);
}

impl Geometry for Point {
    fn transpose(mut self, dx: u32, dy: u32) {
        self.x += dx;
        self.y += dy;
    }
}

impl Geometry for Shape {
    fn transpose(self, dx: u32, dy: u32) {
        for point in self.points.into_iter() {
            point.transpose(dx, dy);
        }
    }
}

impl Parenting for Shape {
    fn give_birth(mut self, child: Point) {
        let boxed = Box::new(child);
        self.points.insert(boxed);
    }
    fn good_bye(mut self, child: Point) {
        self.points.remove(&child);
    }
}

fn main() {
    println!("Hello, world!");
}
