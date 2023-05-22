extern crate hidapi;

use hidapi::HidApi;
use hidapi::HidDevice;
use std::time::Duration;
use std::thread::{sleep};

fn main() {
    let api = HidApi::new().unwrap();
    // This is where you would put your device's Vendor ID and Product ID
    let (vendor_id, product_id) = (0x16c0, 0x05df);
    let device = api.open(vendor_id, product_id).unwrap();

    turn_on_alarm(&device, 30);
}

fn turn_on_alarm(device: &HidDevice, duration_in_seconds: u16) {
    turn_on_relay(&device, 1);
    turn_on_relay(&device, 2);

    sleep(Duration::from_secs(duration_in_seconds.into()));

    turn_off_relay(&device, 1);
    turn_off_relay(&device, 2);
}

fn turn_on_relay(device: &HidDevice, relay_num: u8) {
    let buf = [0x0u8, 0xFF, relay_num];

    let response = device.write(&buf);

    match response {
        Ok(_) => println!("Turn on relay: {}", relay_num),
        Err(err) => eprintln!("Failed to turn on err relay: {} - err: {}", relay_num, err),
    }
}

fn turn_off_relay(device: &HidDevice, relay_num: u8) {
    let buf = [0x0u8, 0xFD, relay_num];

    let response = device.write(&buf);

    match response {
        Ok(_) => println!("Turn on relay: {}", relay_num),
        Err(err) => eprintln!("Failed to turn on err relay: {} - err: {}", relay_num, err),
    }
}