extern crate hidapi;

use hidapi::HidApi;
use hidapi::HidDevice;
use std::ffi::{CString};

pub struct RelayController {
    num: u8,
    device: HidDevice,
}

impl RelayController {
    /// Create a new RelayController.
    ///
    /// This function initializes a new RelayController that controls a USB HID relay device.
    /// The relay device is automatically detected and opened.
    /// 
    /// # Parameters
    ///
    /// * `num`: The number of the relay to be controlled (1..=8). If this is set to 0,
    ///   then the RelayController will control all relays.
    ///
    /// # Returns
    ///
    /// A new instance of `RelayController`, configured to control the specified relay.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to initialize the HIDAPI library, or if it
    /// fails to open the relay device. In a real-world application, these cases should be
    /// handled gracefully.
    #[allow(dead_code)]
    pub fn new(num: u8) -> RelayController {
        let hidapi = HidApi::new().unwrap();

        hidapi.device_list().into_iter().for_each(| device_info | {
            println!(
                "Manufacturer: {} {{Vendor ID: 0x{:04x}}} {{Product ID: 0x{:04x}}} {{Path: {}}}",
                device_info.manufacturer_string().unwrap(),
                device_info.vendor_id(),
                device_info.product_id(),
                device_info.path().to_str().unwrap()
            );
        });

        let device_info = hidapi.device_list().next().unwrap();
        let device = device_info.open_device(&hidapi).unwrap();

        RelayController {
            num: num,
            device: device
        }
    }

    #[allow(dead_code)]
    pub fn new_from_path(num: u8, path: String) -> RelayController {
        let hidapi = HidApi::new().unwrap();
        let c_path = CString::new(path).expect("Failed to convert from String to CString");
        let device = hidapi.open_path(&c_path).unwrap();

        RelayController {
            num: num,
            device: device
        }
    }

    #[allow(dead_code)]
    pub fn new_from_ids(num: u8, vid: u16, pid: u16) -> RelayController {
        let hidapi = HidApi::new().unwrap();
        let device = hidapi.open(vid, pid).unwrap();

        RelayController {
            num: num,
            device: device
        }
    }

    pub fn turn_on(&self) {
        let command = 0xFF;

        match self.num {
            0 => {
                for i in 1..=8 {
                    self.send_command(command, i);
                }
            },
            _ => {
                self.send_command(command, self.num);
            }
        }
    }

    pub fn turn_off(&self) {
        let command = 0xFD;

        match self.num {
            0 => {
                for i in 1..=8 {
                    self.send_command(command, i);
                }
            },
            _ => {
                self.send_command(command, self.num);
            }
        }
    }

    fn send_command(&self, command: u8, relay_num: u8) {
        let command_buf = [0x0u8, command, relay_num];

        let res = self.device.write(&command_buf);
        match res {
            Ok(_) => return,
            Err(err) => eprintln!("Failed in the command sended (num: {}): {}", relay_num, err)
        }
    }
}