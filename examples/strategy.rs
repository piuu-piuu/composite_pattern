// different entities
struct Thermometer;
struct Socket;
struct Bulb;
pub struct Appliances {
    devices: Vec<Box<dyn MainFunction>>,
}

// single interface on client side
trait MainFunction {
    fn start_working(&self);
}

// pool of implementations
impl MainFunction for Thermometer {
    fn start_working(&self) {
        println!("Getting temperature...")
    }
}

impl MainFunction for Socket {
    fn start_working(&self) {
        println!("Providing power...")
    }
}

impl MainFunction for Bulb {
    fn start_working(&self) {
        println!("Lighting the room...")
    }
}

fn main() {
    let appliances = Appliances {
        devices: vec![
            Box::new(Thermometer {}),
            Box::new(Socket {}),
            Box::new(Bulb {}),
        ],
    };

    for device in appliances.devices {
        device.start_working();
    }
}
