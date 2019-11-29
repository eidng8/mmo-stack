/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::io;
use std::net::{IpAddr, SocketAddr, UdpSocket};
use std::thread::Thread;

use rand::Rng;

use crate::binary::Binary;
use crate::commands::ping;

mod binary;
mod commands;

//use std::time;

const SERVER: &str = "127.0.0.1:9000";

fn main() {
    let ip: IpAddr = "127.0.0.1".parse().unwrap();
    let port = rand::thread_rng().gen_range(60000, 65530);
    let addr = SocketAddr::new(ip, port);
    // Set up a new socket on port 9000 to listen on.
    let socket = UdpSocket::bind(&addr).expect("couldn't bind to address");
    println!("socket ready.");

    // Allocate a buffer for it
    let mut buf = [0; 65535];
    ping(&socket);

    // main "game" loop
    println!("entering main loop...");
    loop {
        // wait for user to enter a line
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(bytes_read) => {
                if 0 == bytes_read {
                    // EOF reached, exit the main loop
                    break;
                }

                let line = String::from(line.trim());
                if "exit" == line {
                    break;
                }

                // send the line to server
                // note that `send_to()` blocks
                socket.send_to(&line.into_bytes(), SERVER)
                    .expect("failed to send data to server.");

                // Receives a single datagram message on the socket.
                // If `buf` is too small to hold
                // the message, it will be cut off.
                // not that `recv_from()` blocks
                let (amt, _) = socket.recv_from(&mut buf)
                    .expect("failed to receive data from server");
                println!("server says ({} bytes): {}", amt, String::from_utf8_lossy(&buf));
            }
            Err(error) => println!("error: {}", error)
        }
    }
}

fn ping(socket: &UdpSocket) {
    let ping = ping::Ping::new();
    let mut buf: Vec<u8> = vec![1];
    buf.extend(
        ping.to_binary()
            .expect("failed to convert ping.")
            .iter()
            .clone());
    socket.send_to(&buf, SERVER).expect("failed to send ping.");
}
