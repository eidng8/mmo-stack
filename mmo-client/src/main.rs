/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use chrono::prelude::*;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

const SERVER: &str = "127.0.0.1:9000";

fn main() {
    let mut data = [0 as u8; 128];
    loop {
        println!("Trying to connect to server {}", SERVER);
        match TcpStream::connect(SERVER) {
            Ok(mut stream) => {
                println!("Local address: {:?}", stream.local_addr());
                loop {
                    let msg = Local::now().to_rfc3339();

                    match stream.write(msg.as_bytes()) {
                        Ok(_) => println!("Sent timestamp, awaiting reply..."),
                        Err(e) => {
                            println!("Failed to send data: {}", e);
                            break;
                        }
                    }

                    match stream.read(&mut data) {
                        Ok(n) => {
                            println!(
                                "Got {} bytes reply from server: {}",
                                n,
                                String::from_utf8_lossy(&data[..n])
                            );
                        }
                        Err(e) => {
                            println!("Failed to receive data: {}", e);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
