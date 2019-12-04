/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

//! The `listen()` function provided by different listener modules may seem
//! similar and full of duplicate codes. However, they maybe changed later.
//!
//! -------
//! ## Notes on all `listen()` functions
//! - The bound socket is split into `RecvHalf` and `SendHalf` streams via the
//! `split()` method.
//! - The `RecvHalf` is used by the primary loop, which runs indefinitely,
//! processing all incoming messages from the UDP socket.
//! - A dedicated task is spawned to send message. The `SendHalf` is also
//! *moved* to the dedicated task.
//! - A `mpsc::channel` is created for communicating between the two tasks.
//! - While a message is processed by the primary loop, a `RemoteContent`
//! is generated and send to the `mpsc::channel`.
//! - The task dedicated to sending message loops indefinitely, waiting and
//! sending everything to remote clients.
//!
//!
//! ## Thoughts
//! This is intended to form basic steps of server listeners. So that actual
//! computation works can be left out of network processing stack. In future,
//! the `mpsc::channel` maybe provided somewhere high up in the system
//! hierarchy. However, to the internal of this function, the `mpsc::channel`
//! could be utilized in the same way. Also at the moment, it doesn't seem
//! necessary for this function to utilize mutex. Since actual process of
//! received data would most likely happen somewhere else. This function
//! merely sends all received messages to `mpsc::channel` without any need
//! to concern what the message actually mean. Further more, the receiving
//! and transmitting ends of the `mpsc::channel` don't need to come from the
//! same channel. Though it is not obvious here and right now. It could be
//! that messages received from UDP are pushed to a message processing
//! `mpsc::channel`, while data that needs sending back to client would come
//! from another `mpsc::channel` that holds results of all message processes.

pub mod tcp;
pub mod udp;
