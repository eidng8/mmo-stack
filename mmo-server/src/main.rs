/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */


use crate::listeners::Listeners;

mod binary;
mod commands;
mod game;
mod listeners;

fn main() {
    let ip = "127.0.0.1:9000";
    let addr = ip.parse().expect("Failed to parse socket address.");
    let mut listeners = Listeners::new();
    if let Err(e) = listeners.listen_on_udp(addr) {
        println!("Failed to create socket listener on {}\nError: {:?}", ip, e)
    }

    // Set up a new socket on port 9000 to listen on.
//    let socket = game::prepare_socket(&addr).expect("couldn't prepare server socket");
    println!("server socket is ready.");

    // Initialize poller.
//    let mut poller = game::prepare_event(&socket).expect("couldn't prepare event poller");
    println!("event poller is ready.");

//    game::main_loop(&socket, &mut poller)
}
