use composite_pattern::*;
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

    let mut scene = Scene { shapes: Vec::new() };
    let single_point = Point {
        x: rand::thread_rng().gen_range(1..101),
        y: rand::thread_rng().gen_range(1..101),
    };
    scene.shapes.push(Box::new(single_point));
    scene.shapes.push(Box::new(composite_shape));

    // transposing all elements as if they are the one structure
    // with dynamic dispatch
    for geometry in &mut scene.shapes {
        geometry.transpose(1, 1);
        geometry.transpose(1, 1);
    }
}
