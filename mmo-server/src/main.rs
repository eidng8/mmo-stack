/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */
use crate::listeners::tcp;
use crate::listeners::udp;

mod listeners;

#[tokio::main]
async fn main() {
    tokio::spawn(udp::listen("127.0.0.1:30000"));
    tcp::listen("127.0.0.1:9000").await;

//    loop {}
}
