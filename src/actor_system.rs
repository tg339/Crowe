//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.

use std::thread;
use std::thread::Thread;
use actor_ref::ActorRef;
use actor::Actor;
use rustc_serialize::Decodable;

#[derive(Debug)]
pub struct ActorSystem {
    address: String,
    name: String
    // actors: Vec<ActorRef>
}

#[derive(Debug)]
struct Event <T: Decodable>{
    destinations: Vec<ActorRef>,
    message: T
}

impl ActorSystem {
    /// This method takes a name and address and generates a new actor.
    ///
    ///
    /// This should be refactored to take in a name and a configuration.
    /// 
    fn new(name: String, address: String) -> ActorSystem {
        // let sys_thread = thread::spawn(move || {
        //     println!("Spawed actor system!");
        // });

        ActorSystem {
            name: name,
            address: address
        }
    }

    // fn spawn_actor<T>(&self, name: String, receive: fn()) {
    //     &self.actors.push(T::new(name, receive));
    // }

    // fn broadcast(&self, e: Event) {
    //     for a in &self.actors {
    //         //send to all actors
    //     }
    // }

}

