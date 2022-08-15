// different entities
struct Thermometer;
struct Socket;
struct Bulb;

// single interface on device side
trait State {
    fn device_state(&self);
}

// different implementations
impl State for Thermometer {
    fn device_state(&self) {
        println!("Temperature is 20 deg C")
    }
}

impl State for Socket {
    fn device_state(&self) {
        println!("Socket power is 220V")
    }
}

impl State for Bulb {
    fn device_state(&self) {
        println!("Light power is 25 W")
    }
}

struct Visitor;

impl Visitor {
    fn current_temperature() {
        Thermometer::device_state(&Thermometer {});
    }

    fn socket_power() {
        Socket::device_state(&Socket {});
    }

    fn light_power() {
        Bulb::device_state(&Bulb {});
    }
}

fn main() {
    Visitor::current_temperature();
    Visitor::light_power();
    Visitor::socket_power();
}
