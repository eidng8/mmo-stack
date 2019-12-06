/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::mpsc;

#[derive(Debug)]
struct Content {
    msg: String,
}

#[derive(Debug)]
struct RemoteContent {
    addr: SocketAddr,
    content: Content,
}

struct Connection<'a> {
    iostream: TcpStream,
    istream: ReadHalf<'a>,
    ostream: WriteHalf<'a>,
}

pub struct TcpHandler<'a> {
    bound: SocketAddr,
    listener: Option<TcpListener>,
    tx: mpsc::Sender<RemoteContent>,
    rx: mpsc::Receiver<RemoteContent>,
    connections: HashMap<SocketAddr, Connection<'a>>,
}

impl TcpHandler<'_> {
    pub fn new(addr: &str) -> Self {
        let (tx, rx) = mpsc::channel(1024);

        TcpHandler {
            bound: addr.parse().unwrap(),
            listener: None,
            tx,
            rx,
            connections: HashMap::with_capacity(1024),
        }
    }

    /// Start listening on the given address.
    /// This method binds the given TCP socket and keeps looping indefinitely.
    ///
    /// -------
    ///
    /// ## Arguments
    /// * `addr` - IP address and port to listen on
    pub async fn listen(&'static mut self) {
        self.listener = Some(TcpListener::bind(self.bound).await.unwrap());
        println!("TCP socket ready. [{}]", self.bound);
        // let (tx, rx) = mpsc::channel(1024);
        tokio::spawn(self.send());
        loop {
            if let Some(ref mut listener) = self.listener {
                let (mut stream, remote) = listener.accept().await.unwrap();
                {
                    let (i, o) = stream.split();
                    let connection = Connection {
                        iostream: stream,
                        istream: i,
                        ostream: o,
                    };
                    self.connections
                        .entry(remote)
                        .and_modify(|e| *e = connection)
                        .or_insert(connection);
                    tokio::spawn(self.handle_connection(remote));
                }
            }
        }
    }

    async fn handle_connection(&mut self, remote: SocketAddr) {
        let mut buf = [0; 1024];
        // let (mut r, w) = stream.split();
        // let warc = Arc::new(Mutex::new(w));

        // In a loop, read data from the socket and write the data back.
        loop {
            if let Some(conn) = self.connections.get_mut(&remote) {
                match conn.istream.read(&mut buf).await {
                    Ok(n) if n == 0 => break, // socket closed
                    Ok(n) => self.process(remote.clone(), &buf, n),
                    Err(e) => {
                        println!("Failed to read from TCP socket; err = {:?}", e);
                        break;
                    }
                };
            }
        }
    }

    fn process(&mut self, remote: SocketAddr, buf: &[u8], cb: usize) {
        let got = String::from_utf8_lossy(&buf[..cb]);
        println!("Got data from TCP client {} > {}", remote, got);
        self.tx.send(RemoteContent {
            addr: remote,
            content: Content {
                msg: format!("Got ya [{}]! Ya said {}", remote, got),
            },
        });
    }

    async fn send(&mut self) {
        loop {
            if let Some(rc) = self.rx.recv().await {
                if let Some(io) = self.connections.get_mut(&rc.addr) {
                    let msg = rc.content.msg;
                    match io.ostream.write_all(msg.as_bytes()).await {
                        Ok(_) => println!("Sent back to TCP client < {}", msg),
                        Err(e) => println!("Failed to write to TCP socket; err = {:?}", e),
                    }
                }
            }
        }
    }
}
