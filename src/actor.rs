//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.
use rustc_serialize::Decodable;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;


pub struct Actor<T: Decodable + Debug> {
    pub name: String,
    pub channel: (Sender<T>, Receiver<T>), //Accessible with channel.1
    pub receive: fn(message: T)
}

impl <T: Decodable + Clone + Debug> Actor <T> {
    
    pub fn send(&self, message: T) {
        
        // The transmission end of the channel
        self.channel.0.send(message).unwrap(); 
        
        // The receveiving end of the channel
        let message = self.channel.1.recv().unwrap(); 

        (self.receive)(message);
    }


    pub fn new(name: String, receive: fn(message: T)) -> Actor <T> {
        Actor {
            name: name,
            receive: receive,
            channel: channel()
        }   
    }
}