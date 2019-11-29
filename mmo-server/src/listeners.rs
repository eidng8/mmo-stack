/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use std::cell::RefCell;
use std::io;
use std::net::SocketAddr;

use mio::Token;

use crate::listeners::udp::UdpListener;

pub mod udp;

pub trait Listener {
    fn listen(&mut self) -> Option<bool>;
}

pub struct Listeners {
    pool: RefCell<Vec<Box<dyn Listener>>>,
}

impl Listeners {
    pub fn new() -> Listeners {
        let pool = RefCell::new(Vec::new());
        Listeners { pool }
    }

    pub fn with_capacity(capacity: usize) -> Listeners {
        let pool = RefCell::new(Vec::with_capacity(capacity));
        Listeners { pool }
    }

    pub fn len(&self) -> usize {
        self.pool.borrow().len()
    }

    pub fn listen_on_udp(&self, addr: SocketAddr) -> io::Result<()> {
        let listener = UdpListener::new(addr, self.pool.borrow().len())?;
        self.pool.borrow_mut().push(Box::new(listener));
        Ok(())
    }

    pub fn start(&self, idx: usize) -> Option<bool> {
        if let Some(mut listener) = self.pool.borrow_mut().get(idx) {
            listener.listen();
        }

        None
    }

    pub fn start_all(&self) {
        for mut listener in self.pool.borrow_mut().iter() {
            listener.listen();
        }
    }
}
