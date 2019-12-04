/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use tokio::net::UdpSocket;

pub async fn listen(addr: &str) {
    let socket = UdpSocket::bind(addr).await.unwrap();
    println!("UDP socket ready. [{}]", addr);
    let (mut r, mut w) = socket.split();

    // In a loop, read data from the socket and write the data back.
    loop {
        let got: String;
        let remote;
        let mut buf = [0; 1024];

        match r.recv_from(&mut buf).await {
            Ok((cb, _)) if cb == 0 => return, // socket closed
            Ok((cb, addr)) => {
                remote = addr;
                got = String::from_utf8_lossy(&buf[..cb]).into_owned();
                println!("Got data from UDP client {} > {}", remote, got);
                cb
            }
            Err(e) => {
                println!("Failed to read from UDP socket; err = {:?}", e);
                return;
            }
        };

        // Write the data back
        let msg = format!("{} [{}]", got, remote);
        match w.send_to(msg.as_bytes(), &remote).await {
            Ok(_) => println!("Sent back to UDP client {}  < {}", remote, msg),
            Err(e) => println!("Failed to write to UDP socket; err = {:?}", e),
        }
    }
}
