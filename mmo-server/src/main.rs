/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */
use crate::listeners::tcp::listen;

mod listeners;

#[tokio::main]
async fn main() {
    tokio::spawn(listen("127.0.0.1:9000"));

    loop {}
}
