/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::io;
use std::net::SocketAddr;
use std::thread;

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::UdpSocket;

use crate::listeners::Listeners;

static mut UDP_LISTENERS: Listeners = Listeners::new();

pub fn main_loop() -> io::Result<()> {
    let ip = "127.0.0.1:9000";
    let addr = ip.parse().expect("Failed to parse socket address.");

    unsafe {
        if let Err(e) = UDP_LISTENERS.listen_on_udp(addr) {
            println!("Failed to create socket listener on {}\nError: {:?}", ip, e)
        }
    }

    println!("Start main loop...");
//    thread::spawn(|| process_inbound(socket, poller, 65535));
    loop {}
}
