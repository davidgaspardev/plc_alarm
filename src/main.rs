mod relay;
mod s7;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let relay_control = relay::RelayController::new(0);
    let mut s7_client = s7::S7Client::connect("192.168.0.1".to_string());

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
