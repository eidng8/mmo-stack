/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::net::SocketAddr;
use tokio::net::udp::SendHalf;
use tokio::net::UdpSocket;
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

/// Start listening on the given address.
/// This method binds the given UDP socket and keeps looping indefinitely.
///
/// -------
///
/// ## Arguments
/// * `addr` - IP address and port to listen on
pub async fn listen(addr: &str) {
    let mut buf = [0; 65535];
    let socket = UdpSocket::bind(addr).await.unwrap();
    println!("UDP socket ready. [{}]", addr);

    let (mut r, w) = socket.split();
    let (mut tx, rx) = mpsc::channel(1024);
    tokio::spawn(send(rx, w));

    loop {
        match r.recv_from(&mut buf).await {
            Ok((cb, _)) if cb == 0 => return, // socket closed
            Ok((cb, addr)) => process(&buf, cb, addr, &mut tx).await,
            Err(e) => {
                println!("Failed to read from UDP socket; err = {:?}", e);
                return;
            }
        };
    }
}

/// Process *one* received message. Pushes the message to the `mpsc::channel`.
///
/// ## Arguments
/// * buf - received data
/// * cb - number of bytes received
/// * addr - address of the remote peer
/// * tx - the transmitter of `mpsc::channel`
async fn process(buf: &[u8], cb: usize, addr: SocketAddr, tx: &mut mpsc::Sender<RemoteContent>) {
    let got = String::from_utf8_lossy(&buf[..cb]).into_owned();
    let msg = format!("Got ya [{}]! Ya said {}", addr, got);
    let content = RemoteContent {
        addr,
        content: Content {
            msg: String::from(msg),
        },
    };
    match tx.send(content).await {
        Ok(_) => (),
        Err(e) => println!("Failed to push to message channel: {:?}", e),
    }
    println!("Got data from UDP client {} > {}", addr, got);
}

/// Process message on the `mpsc::channel`, sending them to remote peer.
///
/// --------
///
/// ## Note
/// This function loops indefinitely.
///
/// --------
///
/// ## Arguments
/// * rx - receiver of the `mpsc::channel`
/// * stream - `SendHalf` of the UDP socket.
async fn send(mut rx: mpsc::Receiver<RemoteContent>, mut stream: SendHalf) {
    loop {
        if let Some(remote) = rx.recv().await {
            let addr = remote.addr;
            let msg = remote.content.msg;
            match stream.send_to(msg.as_bytes(), &addr).await {
                Ok(_) => println!("Sent back to UDP client {}  < {}", addr, msg),
                Err(e) => println!("Failed to write to UDP socket; err = {:?}", e),
            }
        }
    }
}
