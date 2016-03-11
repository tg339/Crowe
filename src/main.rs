extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Role};
use crowe::actor_system::ActorSystem;
use rustc_serialize::{Decodable};
use rustc_serialize::json::*;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread::sleep;
use std::time::Duration;
use std::collections::BTreeMap;


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
    fn receive(&self, message: Json) {

        fn add_exclamation(content: String) -> String {
            return content + "!"
        }
        // sleep(Duration::from_millis(2));

        println!("{:?}", message.to_string() + &*self.first_name.clone() + &*self.say_hi().clone() );
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

fn main() {
    // Spawing Actor system with a threadpool of 4
    let mut system = ActorSystem::new(4);

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor("Crowe".to_string(), Box::new(Russel{first_name: "Russel".to_string()}));
    }

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor("Joaquin".to_string(), Box::new(Joaquin{last_name: "Russel".to_string()}));
    }


    let crowe = system.actor_refs.borrow().get("Crowe").unwrap().clone();
    let joaquin = system.actor_refs.borrow().get("Joaquin").unwrap().clone();

    // let some = system.actors.borrow().get("Crowe").unwrap().clone();

    let message = MyMessage{content: "Are you not entertained?".to_string()};
    let message2 = MyMessage{content: "No, I am not entertained".to_string()};

    let response = crowe.send(message.to_json());
    let response2 = joaquin.send(message2.to_json());

    println!("{:?}", response.recv());
    println!("{:?}", response2.recv());


    // Assignement 2 trial division.
    // =============================

    // Definition of the actor roles, we only have workers since the master is the Actor sytem
    // We also will compute the repartition of the tasks on the current thread.
    // The Actor does not need to hold any state
    #[derive(Clone)]
    struct Worker;

    #[derive(RustcDecodable, RustcEncodable)]
    struct DivideOrder {
        divided_n: usize,
        number_list: Vec<usize>
    }

    impl ToJson for DivideOrder {
        fn to_json(&self) -> Json {
            let mut d = BTreeMap::new();

            d.insert("number_list".to_string(), self.number_list.to_json());
            d.insert("divided_n".to_string(), self.divided_n.to_json());
            Json::Object(d)
        }
    }

    impl Role for Worker {
        fn receive(&self, message: Json) {

            let obj = message.as_object().expect("The message should be a valid Json object");

            let number_list = obj.get("number_list")
                .expect("The message received doesn't have a 'number_list' defined.")
                .as_array()
                .expect("The 'number_list' field is not an array");

            let n = obj.get("divided_n")
                .expect("The message received doesn't have a 'divided_n' defined.")
                .as_u64()
                .expect("The 'divided_n' field is not an integer");

            number_list.iter()
                .filter_map(|number| {
                    let temp = number.as_u64().expect("'number_list' contains non number values");

                    if n % temp == 0 {
                        Some(temp)
                    } else {
                        None
                    }
                }).collect::<Vec<u64>>();
        }
    }

    // Execution of the division template
    let number_to_divide: usize = 3293428;
    let processors: usize = 2;
    let mut trialSystem = ActorSystem::new(processors);

    // Let's compute the repartition of the numbers
    // We ceil, the last bucket may have less work to do but it guarantees that
    // all the work will be assigned
    let number_per_worker = ((number_to_divide as f64)/ (processors as f64)).ceil() as usize;
    let mut channels = Vec::new();

    let worker = &mut trialSystem.spawn_actor("Worker".to_string(), Box::new(Worker));

    // Generate the work
    for i in 1..processors {
        let mut work = Vec::with_capacity(number_per_worker);
        let upper_bound = (i * number_per_worker) + 1;
        if upper_bound < number_to_divide{
            for it in ((i - 1) * number_per_worker + 1)..upper_bound {
                work.push(it);
            }
        }
        let divideOrder = DivideOrder{divided_n: number_to_divide, number_list: work};
        channels.push(worker.send(divideOrder.to_json()));
    }
}
