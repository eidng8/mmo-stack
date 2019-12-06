/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::tcp::WriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::{mpsc, Mutex};

#[derive(Debug)]
struct Content {
    msg: String,
}

#[derive(Debug)]
struct RemoteContent<'a> {
    stream: Arc<Mutex<WriteHalf<'a>>>,
    content: Content,
}

/// Start listening on the given address.
/// This method binds the given TCP socket and keeps looping indefinitely.
///
/// -------
///
/// ## Arguments
/// * `addr` - IP address and port to listen on
pub async fn listen(addr: &str) {
    let mut listener = TcpListener::bind(addr).await.unwrap();
    println!("TCP socket ready. [{}]", addr);
    let (tx, rx) = mpsc::channel(1024);
    tokio::spawn(send(rx));

    loop {
        let (stream, remote) = listener.accept().await.unwrap();

        tokio::spawn(handle_connection(remote, stream, &mut tx));
    }
}

async fn handle_connection<'a>(
    remote: SocketAddr,
    mut stream: TcpStream,
    tx: &mut mpsc::Sender<RemoteContent<'a>>,
) {
    let mut got: String;
    let mut buf = [0; 1024];
    let (mut r, w) = stream.split();
    let warc = Arc::new(Mutex::new(w));

    // In a loop, read data from the socket and write the data back.
    loop {
        match r.read(&mut buf).await {
            Ok(n) if n == 0 => break, // socket closed
            Ok(n) => {
                got = String::from_utf8_lossy(&buf[..n]).into_owned();
                println!("Got data from TCP client {} > {}", remote, got);
                tx.send(RemoteContent {
                    stream: Arc::clone(&warc),
                    content: Content { msg: format!("") },
                });
            }
            Err(e) => {
                println!("Failed to read from TCP socket; err = {:?}", e);
                break;
            }
        };
    }
}

async fn send<'a>(mut rx: mpsc::Receiver<RemoteContent<'a>>) {
    loop {
        if let Some(rc) = rx.recv().await {
            let mut stream = rc.stream.lock().await;
            let msg = rc.content.msg;
            match (*stream).write_all(msg.as_bytes()).await {
                Ok(_) => println!("Sent back to TCP client < {}", msg),
                Err(e) => println!("Failed to write to TCP socket; err = {:?}", e),
            }
        }
    }
}
