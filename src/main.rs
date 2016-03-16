extern crate crowe;
extern crate time;
extern crate rustc_serialize;
use crowe::actor::{Role};
use crowe::actor_system::ActorSystem;
use rustc_serialize::{Decodable};
use rustc_serialize::json::*;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread::sleep;
use time::{Duration, PreciseTime};
use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};

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


    // Assignement 2 trial division.
    // =============================

    // Definition of the actor roles, we only have workers since the master is the Actor sytem
    // We also will compute the repartition of the tasks on the current thread.
    // The Actor does not need to hold any state
    println!("====== Assignement 2 =====");

    #[derive(Clone)]
    struct Worker;

    #[derive(RustcDecodable, RustcEncodable)]
    struct DivideOrder {
        divided_n: usize,
        number_upper: usize,
        number_lower: usize
    }

    impl ToJson for DivideOrder {
        fn to_json(&self) -> Json {
            let mut d = BTreeMap::new();

            d.insert("number_upper".to_string(), self.number_upper.to_json());
            d.insert("number_lower".to_string(), self.number_lower.to_json());
            d.insert("divided_n".to_string(), self.divided_n.to_json());
            Json::Object(d)
        }
    }

    impl Role for Worker {
        fn receive(&self, message: Json) -> Json{

            let obj = message.as_object().expect("The message should be a valid Json object");

            let number_upper = obj.get("number_upper")
                .expect("The message received doesn't have a 'number_upper' defined.")
                .as_u64()
                .expect("The 'number_upper' field is not an integer");

            let number_lower = obj.get("number_lower")
                .expect("The message received doesn't have a 'number_lower' defined.")
                .as_u64()
                .expect("The 'number_lower' field is not an integer");

            let n = obj.get("divided_n")
                .expect("The message received doesn't have a 'divided_n' defined.")
                .as_u64()
                .expect("The 'divided_n' field is not an integer");

            return (number_lower..number_upper)
                .filter_map(|number| {

                    if n % number == 0 {
                        Some(number)
                    } else {
                        None
                    }
                }).collect::<Vec<u64>>().to_json();
        }
    }


    // This is to store the mean of the results
    let mut results: HashMap<usize, f64> = HashMap::new();
    let max_processors = 16;
    let total_start = PreciseTime::now();

    // We want to test several times to get the average value of the spent time
    for repetition in 1..20 {
        println!("Iteration {:?}", repetition);
        for processors in 1..max_processors {
            //println!("Speed test with: {:?}", processors);

            // Execution of the division template
            let number_to_divide: usize = 32934280;
            let mut trialSystem = ActorSystem::new(processors);

            // Let's compute the repartition of the numbers
            // We ceil, the last bucket may have less work to do but it guarantees that
            // all the work will be assigned
            let number_per_worker = ((number_to_divide as f64)/ (processors as f64)).ceil() as usize;
            let mut channels = Vec::new();

            let start = PreciseTime::now();

            // We only need to spawn one actor in the system because the execution of the actor
            // is multithreaded in a threadpool.
            let worker = &mut trialSystem.spawn_actor("Worker".to_string(), Box::new(Worker));

            // Generate the work
            for i in 1..(processors + 1) {

                let lower_bound = (i - 1) * number_per_worker + 1;
                let upper_bound = (i * number_per_worker) + 1;

                let divideOrder = DivideOrder{
                    divided_n: number_to_divide,
                    number_upper: upper_bound,
                    number_lower: lower_bound
                 };
                channels.push(worker.send(divideOrder.to_json()));
            }

            for i in 0..processors {
                // Receives the list of numbers factorized. We don't need those for the test
                let res = channels[i].recv().unwrap();
                // To see the results uncomment below
                // println!("Result from processor {0}: {1}", i + 1 , res);
            }

            let elapsed_time = start.to(PreciseTime::now()).num_milliseconds() as f64;

            match results.entry(processors) {
                Occupied(mut entry) => { *entry.get_mut() = (entry.get() + elapsed_time) / 2.0;},
                Vacant(entry) => {entry.insert(elapsed_time);}
            };
            // println!("Total Time to compute: {:?} ms", start.to(PreciseTime::now()).num_milliseconds());
        }
    }
    println!("Test Finished in {:?} seconds", total_start.to(PreciseTime::now()).num_seconds());
    println!("Results: ", );
    for (processor, time_to_execute) in &results {
        println!("Job with {:?} processors executed on average in {} ms", processor, time_to_execute);
    }
}
