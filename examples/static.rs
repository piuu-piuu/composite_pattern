use patterns::*;
use rand::Rng;

fn main() {
    let mut composite_shape = Shape { points: Vec::new() };

    for _ in 1..10 {
        let point = Point {
            x: rand::thread_rng().gen_range(1..101),
            y: rand::thread_rng().gen_range(1..101),
        };
        composite_shape.points.push(point);
    }

    // transposing all elements as if they are the one structure

    composite_shape.transpose(
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101),
    );

    composite_shape.transpose(
        rand::thread_rng().gen_range(1..101),
        rand::thread_rng().gen_range(1..101),
    );
}
