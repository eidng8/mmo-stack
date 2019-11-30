/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */


use crate::listeners::udp;

//mod binary;
//mod commands;
//mod game;
mod listeners;

fn main() {
    if let Ok(thread) = udp::listen("127.0.0.1:9000".parse().unwrap(), 1) {
        thread.join().unwrap();
    }
}
