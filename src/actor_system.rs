//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.
use std::sync::{Arc, Mutex};
use std::fmt::Debug;
use std::thread;
use std::thread::Thread;
use actor_ref::ActorRef;
use actor::Actor;
use rustc_serialize::Decodable;
use std::sync::mpsc::{Sender, Receiver, channel};


#[derive(Debug)]
pub struct ActorSystem <T: Decodable + Clone + Send + Sync + Debug>{
    pub address: String,
    pub name: String,
    // Cloning an arc increases the reference count to the ressource
    // This in conjuction with a Mutex makes the ressource mutable
    // accross multiple threads
    // Here we want to be able to add actors from multiple threads
    pub actors: Vec<ActorRef<T>>
}

impl <T: Decodable + Clone + Send + Sync + Debug> ActorSystem <Actor<T>>{
    /// This method takes a name and address and generates a new actor.
    ///
    ///
    /// This should be refactored to take in a name and a configuration.
    /// 
    pub fn new(name: String, address: String) -> ActorSystem <Actor<T>>{
        ActorSystem {
            name: name,
            address: address,
            actors: Arc::new(Mutex::new(Vec::<Actor<T>>::new()))
        }
    }

    /// Spawn the the actor on a thread
    pub fn spawn_actor (&mut self, name: String, receive: fn(T) -> T) {
        let actors = self.actors.clone();
        
        let actor = Actor::new(name, receive);
        actors.lock().unwrap().push(actor);
        
    }

    pub fn broadcast(&mut self, message: T) {
        let actors = self.actors.clone();
        let actor_lock = actors.lock().unwrap();
        for a in actor_lock.iter() {
            a.ask(message.clone());
        }
    }

    // Because Mutex 
    // pub fn  get_actors (self) -> Vec<Actor<T>> {
    //     let actors = self.actors.clone();
    //     let lock = actors.lock().unwrap();
    //     let actor_vec = lock.iter().collect::<Vec<_>>();

    //     return actor_vec;
    // }

}

