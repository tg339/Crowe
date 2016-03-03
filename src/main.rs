extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Role, Message};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;


#[derive(Debug, RustcDecodable, Clone)]
struct MyMessage {
    content: String
}

impl Message for MyMessage {
    fn content(&self) -> String {
        return self.content.clone();
    }
}

#[derive(Clone)]
struct Russel {
    first_name: String
}

#[derive(Clone)]
struct Joaquin {
    last_name: String
}
    
impl Role for Russel {
    fn receive<M>(message: M) where M: Send + Decodable + Message {

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }

        let content = message.content();
        let exclamated = add_exclamation(content);
        // sleep(Duration::from_millis(2));

        println!("{:?}", exclamated);
    }
    
}


impl Role for Joaquin {
    fn receive<M>(message: M) where M: Send + Decodable + Message {

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }

        let content = message.content();
        let exclamated = add_exclamation(content);

        println!("{:?}", exclamated);
    }
    
}

#[derive(Clone)]
enum Cast {
    Role1(Russel),
    Role2(Joaquin)
}

fn main() {
    // Spawing Actor system with a threadpool of 4
    let mut system = ActorSystem::new(4);

    let actor = Cast::Role1(Russel{first_name: "Russel".to_string()});
    let actor2 = Cast::Role2(Joaquin{last_name: "Russel".to_string()});

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor("Crowe".to_string(), actor);  
    }

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor("Joaquin".to_string(), actor2);  
    }



    let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
    let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

    let message = MyMessage{content: "Are you not entertained?".to_string()};
    let message2 = MyMessage{content: "No, I am not entertained".to_string()};

    let response = crowe.send(Russel::receive, message.clone());
    let response2 = joaquin.send(Joaquin::receive, message2.clone());

}
