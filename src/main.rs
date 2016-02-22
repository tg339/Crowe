extern crate crowe;
extern crate rustc_serialize;
extern crate time;
extern crate rand;
use crowe::actor::{Actor};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;
use time::PreciseTime;

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
    let actor = Actor::new("russel".to_string(), receive);
    let russels_line = Message::new("Are you not entertained?".to_string());
    actor.send(russels_line)

    // -------------------------------------------------------------------
    //                            Benchmarks
    // -------------------------------------------------------------------

    // Bandwidth
    // =========

    // Packet that will be used during tests to send part of the Buffer
    #[derive(RustcDecodable)]
    struct Packet {
        body: Vec<u8>
    }

    let buffer_size = 4*1024*1024;
    // Send part of this buffer in the message
    let buffer = vec![0; buffer_size];
    // Number of times to do the experiment for statistical relevance
    let N = 10000;

    // We are changing the packet size to see how it can affect the bandwidth
    let mut packet_size = 4;
    while packet_size <= buffer_size {

        let start = PreciseTime::now();

        for i in 0..N {
            // Put here send and receive of a messsage from one main thread to another
        }

        // Time between start and now
        let duration = start.to(PreciseTime::now());
        let microseconds = duration.num_microseconds().unwrap() as usize;
        let bw_in_MBs = packet_size * N / (1024*1024) / microseconds;

        println!("Bandwidth {:?} MBs for packet size of {}", bw_in_MBs, packet_size );
        packet_size *= 2;
    }

    // Latency
    // =======

    let to_send = 0b00000001u8; // One in binary reprensation of a byte

    let start = PreciseTime::now();

    for i in 0..N {
        // Send the byte to the Actor and wait for the response
    }

    let duration = start.to(PreciseTime::now());
    let microseconds = duration.num_microseconds().unwrap() as usize;
    let latency = microseconds / (2 * N);
    println!("Latency {:?} μs", latency);

    // Computation time
    // ================

    let u = vec![rand::random::<u8>(); N];
    let v = vec![rand::random::<u8>(); N];
    let mut product = 0.0 as u8;

    let start = PreciseTime::now();
    for i in 0..N {
        product += u[i] * v[i];
    }

    let duration = start.to(PreciseTime::now());
    let microseconds = duration.num_microseconds().unwrap() as usize;
    let time_per_operation = microseconds / (2 * N);
     // There are at least 2 floating point operation in the computation statement
     println!("Time per operation {:?} μs", time_per_operation);
}





// fn main  () {

//     fn populate_chans () -> Vec<(Sender<Message>, Receiver<Message>)> {

//         let mut channels = Vec::new();

//         for _ in 0..10 {
//             let (tx, rx) = channel::<Message>();
//             channels.push((tx, rx));
//         }

//         return channels
//     }

//     let chans = populate_chans();
//     let message = Message::new("Message Sent".to_string());


//     chans[0].0.send(message).unwrap();

//     thread::spawn(move || {
//         let ref reception = chans[0].1;

//         let message = reception.recv().unwrap();
//         println!("{:?}", message.line);
//     }).join().unwrap();
// }
