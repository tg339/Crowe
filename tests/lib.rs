extern crate crowe;
extern crate rustc_serialize;

use crowe::actor::Role;
use crowe::actor_system::ActorSystem;

use rustc_serialize::json::*;


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
        return "I, Russel, said hello".to_string();
    }
} 

#[derive(Clone)]
struct Joaquin {
    last_name: String
}
    
impl Role for Russel {
    fn receive(&self, message: Json) -> Json {
        return Json::String(self.say_hi());
    }   
}


impl Role for Joaquin {
    fn receive(&self, message: Json) -> Json {
        return Json::String("Joaquin received".to_string());
    }
}





#[test]
fn generating_multiple_actors_sending_message_and_getting_responses() {
     let system = ActorSystem::new(4);

    {
        &mut system.spawn_actor("Crowe".to_string(), Box::new(Russel{first_name: "Russel".to_string()}));
    }

    {
        &mut system.spawn_actor("Joaquin".to_string(), Box::new(Joaquin{last_name: "Russel".to_string()}));  
    }

    let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
    let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

    // let some = system.actors.borrow().get("Crowe").unwrap().clone();

    let message = MyMessage{content: "Are you not entertained?".to_string()};
    let message2 = MyMessage{content: "No, I am not entertained".to_string()};

    let response = crowe.send(message.to_json()).recv().unwrap();
    let response2 = joaquin.send(message2.to_json()).recv().unwrap();

    assert_eq!(response, "\"I, Russel, said hello\"".to_string());
    assert_eq!(response2, "\"Joaquin received\"".to_string());
}


// #[test]
// fn sending_message_and_getting_responses() {
//      let system = ActorSystem::new(4);

//     {
//         &mut system.spawn_actor("Crowe".to_string(), Box::new(Russel{first_name: "Russel".to_string()}));
//     }

//     {
//         &mut system.spawn_actor("Joaquin".to_string(), Box::new(Joaquin{last_name: "Russel".to_string()}));  
//     }

//     let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
//     let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

//     // let some = system.actors.borrow().get("Crowe").unwrap().clone();

//     let message = MyMessage{content: "Are you not entertained?".to_string()};
//     let message2 = MyMessage{content: "No, I am not entertained".to_string()};

//     let response = crowe.send(message.to_json()).recv().unwrap();
//     let response2 = joaquin.send(message2.to_json()).recv().unwrap();

//     assert_eq!(response, "\"I, Russel, said hello\"".to_string());
//     assert_eq!(response2, "\"Joaquin received\"".to_string());
// }