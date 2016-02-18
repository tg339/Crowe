extern crate crowe;
extern crate rustc_serialize;
use crowe::actor::{Actor};
use crowe::actor_system::ActorSystem;
use rustc_serialize::Decodable;

#[derive(RustcDecodable)]
struct Message {
    line: String
}

fn receive(message: Message) {
    println!("{:?}", message.line);
}


fn main() {
    let actor = Actor{name:"russel".to_string(), receive: receive};
    let russels_line = Message{line: "Are you not entertained?".to_string()};
    (actor.receive)(russels_line);
}