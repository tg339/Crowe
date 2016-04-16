extern crate crowe;
extern crate time;
extern crate rustc_serialize;
extern crate rand;
use crowe::actor::{Role};
use crowe::actor_system::ActorSystem;
use crowe::checkpoint::*;
use rustc_serialize::json::*;
use time::{PreciseTime};
use std::collections::{BTreeMap, HashMap};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::time::Duration;
use std::thread;
use std::sync::mpsc::channel;
use crowe::assignment2::execute_tests_with_worker_recovery;

fn execute() {
    // println!("====== Assignement 3 =====");

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
    let max_processors = 8;
    let total_start = PreciseTime::now();
    
    let trial_system = ActorSystem::new(max_processors);

    for processors in 1..(max_processors + 1) {

        // Execution of the division template
        let number_to_divide: usize = 32934280;
        // Let's compute the repartition of the numbers
        // We ceil, the last bucket may have less work to do but it guarantees that
        // all the work will be assigned
        let number_per_worker = ((number_to_divide as f64)/ (processors as f64)).ceil() as usize;
        let mut channels = Vec::new();

        let start = PreciseTime::now();

        // We only need to spawn one actor in the system because the execution of the actor
        // is multithreaded in a threadpool.
        let worker = &mut trial_system.spawn_actor("Worker".to_string(), Box::new(Worker));


        create_checkpoints(processors);

        // Generate the work
        for i in 1..(processors + 1) {


            let lower_bound = (i - 1) * number_per_worker + 1;
            let upper_bound = (i * number_per_worker) + 1;

            let divide_order = DivideOrder{
                divided_n: number_to_divide,
                number_upper: upper_bound,
                number_lower: lower_bound
             };

            checkpoint_message(divide_order.to_json(), i);

            channels.push(worker.send(divide_order.to_json()));
        }

        for i in 0..processors {
            // Receives the list of numbers factorized. We don't need those for the test
            let res = worker.safe_receive(&channels[i]).unwrap();
            checkpoint_result(i);

            if i == 4 {
                panic!("this is a terrible mistake!");
            }
            // To see the results uncomment below
            // println!("Result from processor {0}: {1}", i + 1 , res);
        }

        let elapsed_time = start.to(PreciseTime::now()).num_milliseconds() as f64;

        match results.entry(processors) {
            Occupied(mut entry) => { *entry.get_mut() = (entry.get() + elapsed_time) / 2.0;},
            Vacant(entry) => {entry.insert(elapsed_time);}
        };
    }

}


fn recover() {
    // println!("====== Assignement 3 =====");

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
    let max_processors = 8;
    let total_start = PreciseTime::now();
    
    let trial_system = ActorSystem::new(max_processors);

    let mut work_completed = read_checkpoint_main().pop().unwrap();

    for processors in work_completed..(max_processors + 1) {

        // Execution of the division template
        let number_to_divide: usize = 32934280;
        // Let's compute the repartition of the numbers
        // We ceil, the last bucket may have less work to do but it guarantees that
        // all the work will be assigned
        let number_per_worker = ((number_to_divide as f64)/ (processors as f64)).ceil() as usize;
        let mut channels = Vec::new();

        let start = PreciseTime::now();

        // We only need to spawn one actor in the system because the execution of the actor
        // is multithreaded in a threadpool.
        let worker = &mut trial_system.spawn_actor("Worker".to_string(), Box::new(Worker));


        // Generate the work
        for i in work_completed..(processors + 1) {


            let lower_bound = (i - 1) * number_per_worker + 1;
            let upper_bound = (i * number_per_worker) + 1;

            let divide_order = DivideOrder{
                divided_n: number_to_divide,
                number_upper: upper_bound,
                number_lower: lower_bound
             };

            checkpoint_message(divide_order.to_json(), i);

            channels.push(worker.send(divide_order.to_json()));
        }

        for i in work_completed..processors {
            // Receives the list of numbers factorized. We don't need those for the test
            let res = worker.safe_receive(&channels[i-work_completed]).unwrap();
            checkpoint_result(i);
        }

        let elapsed_time = start.to(PreciseTime::now()).num_milliseconds() as f64;

        match results.entry(processors) {
            Occupied(mut entry) => { *entry.get_mut() = (entry.get() + elapsed_time) / 2.0;},
            Vacant(entry) => {entry.insert(elapsed_time);}
        };
    }
}


fn main() {
    // execute_tests_with_worker_recovery() 

    let handle = thread::spawn(move || {
        execute();
    });

    match handle.join() {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<&'static str>() {
                println!("Got an error: {}", e);
                let handle = thread::spawn(move || {
                    recover();
                }).join().ok();

            } else {
                println!("Got an unknown error: {:?}", e);
            }
        },
    }
    
}
