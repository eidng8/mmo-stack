/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use chrono::prelude::*;
use rand::Rng;
use std::env;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("mmo-client <server_ip> [-u]");
        println!("-u means use UDP instead of TCP");
        return;
    }

    if args.len() > 2 && "-u" == args[2] {
        udp(&args[1]);
    } else {
        tcp(&args[1]);
    }
}

fn tcp(server: &str) {
    let mut data = [0 as u8; 128];
    loop {
        println!("[TCP] Trying to connect to server {}", server);
        match TcpStream::connect(server) {
            Ok(mut stream) => {
                println!("[TCP] Local address: {:?}", stream.local_addr());
                loop {
                    let msg = Local::now().to_rfc3339();

                    match stream.write(msg.as_bytes()) {
                        Ok(_) => println!("[TCP] Sent timestamp, awaiting reply..."),
                        Err(e) => {
                            println!("[TCP] Failed to send data: {}", e);
                            break;
                        }
                    }

                    match stream.read(&mut data) {
                        Ok(n) => {
                            println!(
                                "[TCP] Got {} bytes reply from server: {}",
                                n,
                                String::from_utf8_lossy(&data[..n])
                            );
                        }
                        Err(e) => {
                            println!("[TCP] Failed to receive data: {}", e);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
            Err(e) => {
                println!("[TCP] Failed to connect: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}

fn udp(server: &str) {
    let socket: UdpSocket;
    let mut data = [0 as u8; 128];
    let mut rng = rand::thread_rng();
    let ip = "127.0.0.1".parse().unwrap();
    loop {
        let port = rng.gen_range(60000, 65530);
        let addr = SocketAddr::new(ip, port);
        match UdpSocket::bind(&addr) {
            Ok(s) => {
                socket = s;
                break;
            }
            _ => (),
        }
    }

    loop {
        println!("[UDP] Trying to connect to server {}", server);
        match socket.connect(server) {
            Ok(_) => {
                println!("[UDP] Local address: {:?}", socket.local_addr());
                loop {
                    let msg = Local::now().to_rfc3339();

                    match socket.send(msg.as_bytes()) {
                        Ok(_) => println!("[UDP] Sent timestamp, awaiting reply..."),
                        Err(e) => {
                            println!("[UDP] Failed to send data: {}", e);
                            break;
                        }
                    }

                    match socket.recv(&mut data) {
                        Ok(n) => {
                            println!(
                                "[UDP] Got {} bytes reply from server: {}",
                                n,
                                String::from_utf8_lossy(&data[..n])
                            );
                        }
                        Err(e) => {
                            println!("[UDP] Failed to receive data: {}", e);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            }
            Err(e) => {
                println!("[UDP] Failed to connect: {}", e);
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
