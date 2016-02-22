//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.
use rustc_serialize::Decodable;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;
use std::thread;


pub struct Actor<T: Decodable + Clone + Send + Sync + Debug> {
    pub name: String,
    pub transmitter: Sender<T>, //Accessible with channel.1
    pub receive: fn(message: T) -> T
}

// Messages must be decodable, clonable and printable with debug
impl <T: Decodable + Clone + Send + Debug + Sync + 'static> Actor <T> {
    
    pub fn ask(&self, message: T) {
        let tx = self.transmitter.clone();
        thread::spawn(move || {
            // let message = self.channel.1.recv().unwrap(); 
            let result = (self.receive)(message);
            tx.send(message.clone()).unwrap();
        });
    }


    pub fn new(name: String, receive: fn(message: T) -> T) -> Actor <T> {
        let (tx, rx) = channel();

        Actor {
            name: name,
            receive: receive,
            transmitter: tx
        }   
    }
}