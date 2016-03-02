use rustc_serialize::Decodable;
use std::fmt::Debug;

pub trait Actor {
    fn receive<M>(message: M) where M: Send + Decodable + Debug + 'static + Sized;
}

