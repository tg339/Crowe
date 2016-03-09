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
    fn receive(message: Json) {

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }

        
        // sleep(Duration::from_millis(2));

        println!("{:?}", message.to_string());
    }

}


impl Role for Joaquin {
    fn receive(message: Json){

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
<<<<<<< HEAD
        let act_ref = &mut system.spawn_actor("Crowe".to_string(), actor);
=======
        let act_ref = &mut system.spawn_actor("Crowe".to_string(), Cast::Role1(Russel{first_name: "Russel".to_string()}));
>>>>>>> actualSytem
    }

    {
        // Spawn as many actors as you want
<<<<<<< HEAD
        let act_ref = &mut system.spawn_actor("Joaquin".to_string(), actor2);
=======
        let act_ref = &mut system.spawn_actor("Joaquin".to_string(), Cast::Role2(Joaquin{last_name: "Russel".to_string()}));  
>>>>>>> actualSytem
    }


    let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
    let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

    let message = MyMessage{content: "Are you not entertained?".to_string()};
    let message2 = MyMessage{content: "No, I am not entertained".to_string()};

    let response = crowe.send(Russel::receive, message.to_json());
    let response2 = joaquin.send(Joaquin::receive, message2.to_json());


    // Assignement 2 trial division.
    // =============================

    // Definition of the actor roles, we only have workers since the master is the Actor sytem
    // We also will compute the repartition of the tasks on the current thread.
    #[derive(Clone)]
    struct Worker {}

    impl Role for Worker {
        fn receive<M>(message: M) where M: Send + Decodable + Message {
            match message.as_object() {
                Some(obj) => match obj.get("prime_list") {
                    Some(prime_list) => match prime_list.as_array() {
                        Some(prime_array) => prime_array.iter().fold(Vec::new(),|acc, js_value| {
                            match js_value.as_i64() {
                                Some(number) => {
                                    if ()
                                }
                                None => acc
                            }
                        })
                        None panic!("The 'prime_list' field is not an array");
                    }
                    None => panic!("The message received doesn't have a 'prime_list' defined.");
                }
                None => panic!("Oops the message received is not an object !");
            }
        }
    }


    // Execution of the division template

    let processors = 2;
    let mut trialSystem = ActorSystem::new(processors)

    let master = Cas
    // The number of

}
