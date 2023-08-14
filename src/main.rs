mod relay;
mod s7;

use std::env;
use std::thread::sleep;
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Please provide the IP address as a command line argument");
    }

    let ip_address = &args[1];
    let mut s7_client = s7::S7Client::connect(ip_address.to_string(), 0, 1);

    let relay_control = relay::RelayController::new(0);

    println!("\"production speed\",\"diameter X\",\"diameter Y\",\"date\"");

    let mut index: u32 = 0;
    let mut prod_speed_bytes = s7_client.read_dword(1, 1088);
    let mut prod_speed = bytes_to_float32(prod_speed_bytes);

    loop {
        if (index % 32) == 0 {
            prod_speed_bytes = s7_client.read_dword(1, 1088);
            prod_speed = bytes_to_float32(prod_speed_bytes);
        }

        let (dia_x_byte, dia_y_byte) = s7_client.read_double_dword(1, 1132);
        let (
            dia_x,
            dia_y
        ) = (
            bytes_to_float32(dia_x_byte),
            bytes_to_float32(dia_y_byte)
        );

        if cfg!(debug_assertions) {
            println!("[?] Production speed: {:?}", prod_speed);
            println!("[?] Diameter X: {:?}", dia_x);
            println!("[?] Diameter Y: {:?}", dia_y);
        }

        if prod_speed > 30.0 && (dia_x < 2.0 || dia_x > 2.15 || dia_y < 4.95 || dia_y > 5.15) {
            // Convert to UNIX timestamp (seconds since 1970-01-01 00:00:00)
            if let Ok(now) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)  {
                println!("{},{},{},{}", prod_speed, dia_x, dia_y, now.as_secs());
            }

            // Turn on/off relay
            relay_control.turn_on();
            sleep(Duration::from_secs(10));
            relay_control.turn_off();
        }

        if index == u32::MAX {
            index = 0;
        } else {
            index += 1;
        }
    }
}

fn bytes_to_float32(bytes: [u8; 4]) -> f32 {
    let parsed_hex = u32::from_be_bytes(bytes);
    f32::from_bits(parsed_hex)
}