extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Actor};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;


// struct Message {
//     line: String
// }

// fn receive(message: Message) {
//     println!("{:?}", message.line);
// }


// fn main() {
//     let actor = Actor{name:"russel".to_string(), receive: receive};
//     let russels_line = Message{line: "Are you not entertained?".to_string()};
//     (actor.receive)(russels_line);
// }


use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(RustcDecodable)]
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


fn main  () {

    fn populate_chans () -> Vec<(Sender<Message>, Receiver<Message>)> {
    
        let mut channels = Vec::new();
        
        for _ in 0..10 {
            let (tx, rx) = channel::<Message>();
            channels.push((tx, rx));
        }
        
        return channels
    }
    
    let chans = populate_chans();
    let message = Message::new("Message Sent".to_string());

    
    chans[0].0.send(message).unwrap();
    
    thread::spawn(move || {
        let ref reception = chans[0].1;

        let message = reception.recv().unwrap();
        println!("{:?}", message.line);
    }).join().unwrap();
}