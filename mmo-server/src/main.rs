/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

mod listeners;

#[tokio::main]
async fn main() {
    // Spawn a task to handle UDP requests.
    tokio::spawn(crate::listeners::udp::listen("127.0.0.1:30000"));

    // Start TCP handling, and blocks the main thread.
    // crate::listeners::tcp::listen("127.0.0.1:9000").await;
    let mut tcp = crate::listeners::tcp_struct::TcpHandler::new("127.0.0.1:9000");
    tcp.listen().await;
}
