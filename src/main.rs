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
    let mut s7_client = s7::S7Client::connect(ip_address.to_string(), 0, 1);

    let relay_control = relay::RelayController::new(0);

    loop {
        let prod_speed_bytes = s7_client.read_dword(1, 1088);
        let (dia_x_byte, dia_y_byte) = s7_client.read_double_dword(1, 1132);

        let (
            prod_speed,
            dia_x,
            dia_y
        ) = (
            bytes_to_float32(prod_speed_bytes),
            bytes_to_float32(dia_x_byte),
            bytes_to_float32(dia_y_byte)
        );

        if cfg!(debug_assertions) {
            println!("[?] Production speed: {:?}", prod_speed);
            println!("[?] Diameter X: {:?}", dia_x);
            println!("[?] Diameter Y: {:?}", dia_y);
        }

        if prod_speed > 30f32 && (dia_x < 2 || dia_x > 2.15 || dia_y < 4.95 || dia_y > 5.15) {

            // Turn on/off relay
            relay_control.turn_on();
            sleep(Duration::from_secs(10));
            relay_control.turn_off();
        }
    }
}

fn bytes_to_float32(bytes: [u8; 4]) -> f32 {
    let parsed_hex = u32::from_be_bytes(bytes);
    f32::from_bits(parsed_hex)
}