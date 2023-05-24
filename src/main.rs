mod relay;
mod s7;

use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide the IP address as a command line argument");
    }

    let ip_address = &args[1];
    let mut s7_client = s7::S7Client::connect(ip_address.to_string());

    let relay_control = relay::RelayController::new(0);

    loop {
        let production_speed = s7_client.read_dword(1, 1088);
        let (diameter_x, diameter_y) = s7_client.read_double_dword(1, 1132);

        if cfg!(debug_assertions) {
            println!("production_speed: {}", production_speed);
            println!("diameter x: {}", diameter_x);
            println!("diameter y: {}", diameter_y);
        }

        if production_speed > 100 && (diameter_x < 2 || diameter_y < 2) {
            relay_control.turn_on();
            sleep(Duration::from_secs(10));
            relay_control.turn_off();
        }
    }
}
