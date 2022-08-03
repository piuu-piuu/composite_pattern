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

    // transposing all points as if they are the one structure

    composite_shape.transpose(
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101),
    );
}
