extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Actor, Message};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;

#[derive(Clone)]
struct Russel  {
    name: String
}

struct Joaquin  {
    name: String
}

#[derive(Debug, RustcDecodable, Clone)]
struct MyMessage {
    content: String
}

impl Message for MyMessage {
    fn content(&self) -> String {
        return self.content.clone();
    }
}



impl Actor for Russel {
    fn receive<M>(message: M) where M: Send + Debug + 'static + Sized + Message {

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }

        let content = message.content();
        let exclamated = add_exclamation(content);

        println!("{:?}", exclamated);
    }

    fn name(&self) -> String {
        return self.name.clone();
    }
    
}

fn main() {
    // Spawing Actor system with a threadpool of 4
    let mut system = ActorSystem::new(4);

    let actor = Russel{name: "Crowe".to_string()};

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor(actor);    
    }

    let actor_ref = system.actor_refs.get("Crowe").unwrap();
    let message = MyMessage{content: "Are you not entertained?".to_string()};

    let response = actor_ref.send(&system.pool, Russel::receive, message);
    println!("{:?}", response);   
}
