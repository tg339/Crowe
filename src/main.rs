extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Actor};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

struct Russel {
    name: String
}

#[derive(Debug, RustcDecodable, Clone)]
struct Message {
    content: String
}

impl Actor for Russel {
    fn receive<Message>(message: Message) {
        let m = message;
    }
}

fn receive(message: Message) { 
    println!("{:?}", message.content);   
}


fn main() {
    let mut system = ActorSystem::<Russel>::new(4);
    {
        let sys = &mut system.spawn_actor(Russel{name: "Crowe".to_string()});    
    }
    let actor_ref = system.actor_refs.first().unwrap();
    let message = Message{content: "Are you not entertained?".to_string()};
    let response = actor_ref.send(&system.pool, receive, message);
    println!("{:?}", response);
    
    
    
}