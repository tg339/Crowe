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
pub struct ActorSystem <T: 'static>{
    address: String,
    name: String,
    actors: Arc<Mutex<Vec<T>>>
}

impl <T: Decodable + Copy + Send + Debug> ActorSystem <Actor<T>>{
    /// This method takes a name and address and generates a new actor.
    ///
    ///
    /// This should be refactored to take in a name and a configuration.
    /// 
    fn new(name: String, address: String) -> ActorSystem <Actor<T>>{
        // let sys_thread = thread::spawn(move || {
        //     println!("Spawed actor system!");
        // });

        ActorSystem {
            name: name,
            address: address,
            actors: Arc::new(Mutex::new(Vec::<Actor<T>>::new()))
        }
    }

    /// Spawn the the actor on a thread
    fn spawn_actor (&mut self, name: String, channel: (Sender<T>, Receiver<T>), receive: fn(T)) {
        let actors = self.actors.clone();
        thread::spawn(move || {
            let actor = Actor::new(name, receive);
            actors.lock().unwrap().push(actor);
        }).join().unwrap();
    }

    // fn broadcast(&self, message: T) {
    //     let actors = self.actors.clone();
    //     for a in actors.lock().unwrap() {
    //         //send to all actors
    //         a.send(message)
    //     }
    // }

}

