mod relay;
mod s7;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    let relay_control = relay::RelayController::new(0);
    let mut s7_client = s7::S7Client::connect("192.168.0.1".to_string());

    println!("read_dword: {}", s7_client.read_dword(1, 1132));

    loop {
        s7_client.read_dword(1, 1132);

        relay_control.turn_on();
        sleep(Duration::from_secs(10));
        relay_control.turn_off();
    }
}
