/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::{io, thread};
use std::net::SocketAddr;
use std::thread::JoinHandle;

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::UdpSocket;

pub fn listen(addr: SocketAddr, token: usize) -> io::Result<JoinHandle<()>> {
    let tok = Token(token);
    let mut socket = UdpSocket::bind(&addr)?;
    let mut events = Events::with_capacity(1024);
    let mut buffer = [0; 65535];

    let poller = Poll::new()?;
    poller.register(&socket, tok, Ready::readable(), PollOpt::edge())?;

    Ok(thread::spawn(move || {
        loop {
            // Poll if we have events waiting for us on the socket.
            println!("polling...");
            if let Err(e) = poller.poll(&mut events, None) {
                println!("Failed to poll event: {:?}", e);
            }

            println!("got events...");
            // If we do iterate through them
            for event in events.iter() {
                println!("event {:?}", event);
                // Validate the token we registered our socket with,
                // in this example it will only ever be one but we
                // make sure it's valid none the less.
                match event.token() {
                    Token(tok) => {
                        if token == tok {
                            read_socket(&mut socket, &mut buffer[..], token);
                        } else {
                            println!("Invalid token: {}", tok);
                        }
                    }
                }
            }
        }
    }))
}

fn read_socket(socket: &mut UdpSocket, buffer: &mut [u8], token: usize) {
    println!(
        "Start processing inbound connections (token {}) ...",
        token
    );
    loop {
        // In this loop we receive from the socket as long as we
        // can read data
        match socket.recv_from(buffer) {
            Ok((bytes_ready, from_addr)) => {
                println!(
                    "listener ({}): client [{}] says ({} bytes) {}",
                    token,
                    from_addr,
                    bytes_ready,
                    String::from_utf8_lossy(buffer)
                );
                // Send the data right back from where it came from.
                socket
                    .send_to(&buffer[..bytes_ready], &from_addr)
                    .expect("failed to send message");
            }
            Err(e) => {
                // If we failed to receive data we have two cases
                if e.kind() == io::ErrorKind::WouldBlock {
                    // If the reason was `WouldBlock` we know
                    // our socket has no more data to give so
                    // we can return to the poll to wait politely.
                    break;
                } else {
                    // If it was any other kind of error, something
                    // went wrong and we terminate with an error.
                    println!("Error occurred: {:?}", e);
                    break;
                }
            }
        }
    }
}
