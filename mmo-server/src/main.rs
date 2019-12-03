/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */
use tokio::net::TcpListener;
use tokio::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("127.0.0.1:9000").await?;
    println!("Server socket ready.");

    loop {
        let (mut stream, addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut got: String;
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                match stream.read(&mut buf).await {
                    Ok(n) if n == 0 => return, // socket closed
                    Ok(n) => {
                        got = String::from_utf8_lossy(&buf[..n]).into_owned();
                        println!("Got data from client {} > {}", addr, got);
                        n
                    }
                    Err(e) => {
                        println!("Failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                let msg = format!("{} [{}]", got, addr);
                match stream.write_all(msg.as_bytes()).await {
                    Ok(_) => {
                        println!("Sent back to client {}  < {}", addr, msg)
                    }
                    Err(e) => println!("Failed to write to socket; err = {:?}", e),
                }
            }
        });
    }
}
