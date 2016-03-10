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
        let act_ref = &mut system.spawn_actor("Crowe".to_string(), Cast::Role1(Russel{first_name: "Russel".to_string()}));
    }

    {
        // Spawn as many actors as you want
        let act_ref = &mut system.spawn_actor("Joaquin".to_string(), Cast::Role2(Joaquin{last_name: "Russel".to_string()}));
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
    // The Actor does not need to hold any state
    #[derive(Clone)]
    struct Worker;

    #[derive(RustcDecodable, RustcEncodable)]
    struct DivideOrder {
        divided_n: u32,
        number_list: Vec<u32>
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
        fn receive(message: Json) {
            match message.as_object() {
                Some(obj) => match (obj.get("number_list"), obj.get("divided_n")) {
                    (Some(number_list), Some(n)) => match number_list.as_array() {
                        Some(number_array) => number_array.iter().fold(Vec::new(),|acc, js_value| {
                            match js_value.as_i64() {
                                Some(number) => {
                                    if n % number == 0 {
                                        acc.push(number);
                                    }
                                    acc;
                                },
                                None => acc
                            }
                        }),
                        None => panic!("The 'number_list' field is not an array")
                    },
                    (None, Some(n)) => panic!("The message received doesn't have a 'number_list' defined."),
                    (Some(l), None) => panic!("The message received doesn't have a 'divided_n' defined."),
                    (None, None) => panic!("The message received doesn't have 'divided_n' or 'number_list' defined."),
                },
                None => panic!("Oops the message received is not an object !")
            }
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
    let work_holder = Vec::with_capacity(processors);

    // Generate the work
    for i in 1..processors {
        work_holder.push(Vec::with_capacity(number_per_worker));
        let upper_bound = (i * number_per_worker) + 1;
        if upper_bound < number_to_divide{
            for it in ((i - 1) * number_per_worker + 1)..upper_bound {
                work_holder[i].push(it);
            }
        }
    }

    // Compute the results
    let channels = Vec::new();
    {
        let act_ref = &mut trialSystem.spawn_actor("Worker".to_string(), Cast::Project(Worker{}));
        for i in 1..processors {
            let divideOrder = DivideOrder{divided_n: number_to_divide, number_list: work_holder[i]};
            channels.push(act_ref.send(Russel::receive, divideOrder.to_json()));
        }
    }
}
