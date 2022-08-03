use std::collections::HashSet;

use composite_pattern::*;
use rand::Rng;

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

    let mut scene: Vec<Box<dyn Geometry>> = Vec::new();
    let single_point = Point {
        x: rand::thread_rng().gen_range(1..101),
        y: rand::thread_rng().gen_range(1..101),
    };

    scene.push(Box::new(single_point));
    scene.push(Box::new(composite_shape));
    for geometry in &scene {
        //
        //DOESN'T WORK
        //
        geometry.transpose(1, 1);
    }
}
