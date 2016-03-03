use rustc_serialize::Decodable;
use std::fmt::Debug;

pub trait Message {
    fn content(&self) -> String;
}

pub trait Role {
    fn receive<M>(message: M) where  M: Send + Decodable  + Message;
}

