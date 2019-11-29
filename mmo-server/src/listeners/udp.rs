/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::{io, thread};
use std::net::SocketAddr;
use std::thread::JoinHandle;

use mio::{Events, Poll, PollOpt, Ready, Token};
use mio::net::UdpSocket;

use crate::listeners::Listener;

pub struct UdpListener {
    addr: SocketAddr,
    token: usize,
    socket: UdpSocket,
    poller: Poll,
    events: Events,
    buffer: Vec<u8>,
    handle: Option<JoinHandle<()>>,
    listening: bool,
}

impl Listener for UdpListener {
    fn listen(&mut self) -> Option<bool> {
        if self.listening {
            return None;
        }

        self.handle = Some(thread::spawn(move|| {
            self.listening = true;
            loop {
                // Poll if we have events waiting for us on the socket.
                match self.poller.poll(&mut self.events, None) {
                    Ok(_) => (),
                    Err(e) => println!("Failed to pool event: {:?}", e),
                }

                // If we do iterate through them
                for event in self.events.iter() {
                    // Validate the token we registered our socket with,
                    // in this example it will only ever be one but we
                    // make sure it's valid none the less.
                    match event.token() {
                        Token(token) => {
                            if token == self.token {
                                self.read_socket();
                            } else {
                                println!("Invalid token: {}", token);
                            }
                        }
                    }
                }
            }
        }));

        Some(true)
    }
}

impl UdpListener {
    pub fn new(addr: SocketAddr, token: usize) -> io::Result<UdpListener> {
        let tok = Token(token);
        let socket = UdpSocket::bind(&addr)?;
        let events = Events::with_capacity(1024);
        let mut buffer: Vec<u8> = Vec::with_capacity(65535);

        let poller = Poll::new()?;
        poller.register(&socket, tok, Ready::readable(), PollOpt::edge())?;

        Ok(UdpListener {
            addr,
            token,
            socket,
            poller,
            events,
            buffer,
            handle: None,
            listening: false,
        })
    }

    fn read_socket(&mut self) {
        println!(
            "Start processing inbound connections (token {}) ...",
            self.token
        );
        loop {
            // In this loop we receive from the socket as long as we
            // can read data
            match self.socket.recv_from(&mut self.buffer) {
                Ok((n, from_addr)) => {
                    println!(
                        "listener ({}): client [{}] says ({} bytes) {}",
                        self.token,
                        from_addr,
                        n,
                        String::from_utf8_lossy(&self.buffer)
                    );
                    // Send the data right back from where it came from.
                    self.socket
                        .send_to(&self.buffer[..n], &from_addr)
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
}
