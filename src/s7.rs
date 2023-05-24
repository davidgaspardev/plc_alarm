extern crate s7;

use s7::{client::Client, tcp, transport::Connection};
use std::net::{IpAddr, Ipv4Addr};
use std::str::FromStr;
use std::time::Duration;

pub struct S7Client {
    addr: IpAddr,
    client: Client<tcp::Transport>
}

impl S7Client {
    pub fn connect(ip: String, rack: u16, slot: u16) -> S7Client {
        let addr_v4 = Ipv4Addr::from_str(&ip).unwrap();
        let addr = IpAddr::from(addr_v4);

        let mut opts = tcp::Options::new(addr, rack, slot, Connection::PG);
        opts.read_timeout = Duration::from_secs(2);
        opts.write_timeout = Duration::from_secs(2);

        let transp = tcp::Transport::connect(opts).unwrap();
        let client = Client::new(transp).unwrap();

        S7Client {
            addr,
            client
        }
    }

    pub fn reconnect(&mut self) {
        let mut opts = tcp::Options::new(self.addr, 5, 5, Connection::PG);
        opts.read_timeout = Duration::from_secs(2);
        opts.write_timeout = Duration::from_secs(2);

        let transp = tcp::Transport::connect(opts).unwrap();
        let client = Client::new(transp).unwrap();

        self.client = client;
    }

    pub fn read_dword(&mut self, db_num: i32, addr: i32) -> i32 {
        let mut buffer = vec![0x0u8; 4];
        let result = self.client.ag_read(db_num, addr, 4, &mut buffer);

        match result {
            Ok(_) => {
                return i32::from_le_bytes([
                    buffer[0],
                    buffer[1],
                    buffer[2],
                    buffer[3]
                ]);
            },
            Err(err) => {
                eprintln!("Failed to read dword: {}", err);

                self.reconnect();
                return self.read_dword(db_num, addr);
            }
        }
    }

    pub fn read_double_dword(&mut self, db_num: i32, addr: i32) -> (i32, i32) {
        let mut buffer = vec![0x0u8; 8];
        let result = self.client.ag_read(db_num, addr, 8, &mut buffer);

        match result {
            Ok(_) => {
                return (
                    i32::from_le_bytes([
                        buffer[0],
                        buffer[1],
                        buffer[2],
                        buffer[3]
                    ]),
                    i32::from_le_bytes([
                        buffer[4],
                        buffer[5],
                        buffer[6],
                        buffer[7]
                    ])
                )
            },
            Err(err) => {
                eprintln!("Failed to read double dword: {}", err);

                self.reconnect();
                return self.read_double_dword(db_num, addr);
            }
        }
    }
}