//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.

use rustc_serialize::Decodable;

#[derive(Debug)]
pub struct Actor<T: Decodable> {
    pub name: String,
    pub receive: fn(message: T)
}






