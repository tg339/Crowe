extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Actor};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;


use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(RustcDecodable, Debug, Clone)]
struct Message {
    line: String
}

impl Message {
    fn new(line: String) -> Message {
        Message {
            line: line
        }
    }
}

fn receive(message: Message) {
    println!("{:?}", message.line);
}


fn main() {
    let system_name = "main".to_string();
    let system_address = "local".to_string();
    let mut system = ActorSystem::new(system_name, system_address);

    let russels_line = Message::new("Are you not entertained?".to_string());

    system.spawn_actor("russel".to_string(), receive);
    system.spawn_actor("jackie".to_string(), receive);

    let actors = system.actors.clone();
    let lock = actors.lock().unwrap();
    
    let actor_array = lock.iter().collect::<Vec<_>>();

    for a in actor_array {
        a.send(russels_line.clone());    
    }
}