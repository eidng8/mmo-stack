/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use tokio::net::TcpListener;
use tokio::prelude::*;

/// Start listening on the given address.
/// This method binds the given UDP socket and keeps looping indefinitely.
///
/// -------
///
/// ## Arguments
/// * `addr` - IP address and port to listen on
pub async fn listen(addr: &str) {
    let mut listener = TcpListener::bind(addr).await.unwrap();
    println!("TCP socket ready. [{}]", addr);

    loop {
        let (mut stream, remote) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut got: String;
            let mut buf = [0; 1024];
            let (mut r, mut w) = stream.split();

            // In a loop, read data from the socket and write the data back.
            loop {
                match r.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // socket closed
                    Ok(n) => {
                        got = String::from_utf8_lossy(&buf[..n]).into_owned();
                        println!("Got data from TCP client {} > {}", remote, got);
                    }
                    Err(e) => {
                        println!("Failed to read from TCP socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                let msg = format!("{} [{}]", got, remote);
                match w.write_all(msg.as_bytes()).await {
                    Ok(_) => println!("Sent back to TCP client {}  < {}", remote, msg),
                    Err(e) => println!("Failed to write to TCP socket; err = {:?}", e),
                }
            }
        });
    }
}
