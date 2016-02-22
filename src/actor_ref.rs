//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.
use rustc_serialize::Decodable;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;

pub struct ActorRef<T: Decodable + Clone + Send + Sync + Debug> {
    pub name: String,
    pub channel: (Sender<T>, Receiver<T>),
    pub path: String //Must be unique
}

// impl ActorRef {
//     fn send(message: T) {
//         if mail_box.is_empty {

//         }
//     }
// }