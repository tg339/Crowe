extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Role};
use crowe::actor_system::ActorSystem;
use rustc_serialize::{Decodable};
use rustc_serialize::json::*;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;


#[derive(RustcDecodable, RustcEncodable)]
struct MyMessage {
    content: String
}

impl ToJson for MyMessage {
    fn to_json(&self) -> Json {
        Json::String(format!("{}", self.content))
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
    fn receive(&self, message: Json) {

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }

        
        // sleep(Duration::from_millis(2));

        println!("{:?}", message.to_string());
    }
    
}


impl Role for Joaquin {
    fn receive(&self, message: Json){

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }

        println!("{:?}", message.to_string());
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

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor("Crowe".to_string(), Cast::Role1(Russel{first_name: "Russel".to_string()}), Box::new(Russel{first_name: "Russel".to_string()}));
    }

    // {
    //     // Spawn as many actors as you want
    //     let act_ref = &mut system.spawn_actor("Joaquin".to_string(), Cast::Role2(Joaquin{last_name: "Russel".to_string()}));  
    // }


    let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
    // let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

    // let some = system.actors.borrow().get("Crowe").unwrap().clone();

    let message = MyMessage{content: "Are you not entertained?".to_string()};
    let message2 = MyMessage{content: "No, I am not entertained".to_string()};

    let response = crowe.send(message.to_json());
    println!("{:?}", response.recv());
    // let response2 = joaquin.send(message2.to_json());

}
