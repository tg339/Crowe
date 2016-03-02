use rustc_serialize::Decodable;
use std::fmt::Debug;

pub trait Message {
    fn content(&self) -> String;
}

pub trait Actor: Clone {
    fn receive<M>(message: M) where M: Send + Debug + 'static + Sized + Message;
    fn name(&self) -> String;
}

