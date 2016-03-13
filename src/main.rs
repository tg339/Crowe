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

impl Russel {
    fn say_hi(&self) -> String {
        return "I said hello".to_string();
    }
}

#[derive(Clone)]
struct Joaquin {
    last_name: String
}
    
impl Role for Russel {
    fn receive(&self, message: Json) -> Json {

        return Json::String("Russel received".to_string());
    }   
}


impl Role for Joaquin {
    fn receive(&self, message: Json) -> Json {
        return Json::String("Joaquin received".to_string());
    }
}

fn main() {
    let system = ActorSystem::new(4);

    {
        // Spawn as many actors as you want
        &mut system.spawn_actor("Crowe".to_string(), Box::new(Russel{first_name: "Russel".to_string()}));
    }

    {
        // Spawn as many actors as you want
        &mut system.spawn_actor("Joaquin".to_string(), Box::new(Joaquin{last_name: "Russel".to_string()}));  
    }


    let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
    let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

    // let some = system.actors.borrow().get("Crowe").unwrap().clone();

    let message = MyMessage{content: "Are you not entertained?".to_string()};
    let message2 = MyMessage{content: "No, I am not entertained".to_string()};
    let message3 = MyMessage{content: "How dare you show your back to me!?".to_string()};

    let response = crowe.send(message.to_json());
    let response2 = joaquin.send(message2.to_json());
    let response3 = joaquin.send_to("Crowe".to_string(), message3.to_json());

    println!("{:?}", response.recv().unwrap());
    println!("{:?}", response2.recv().unwrap());
    println!("{:?}", response3.recv().unwrap());

}
