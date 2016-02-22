//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.

// use std::collections::LinkedList;
// use actor_system::Event;


#[derive(Debug)]
pub struct ActorRef {
    pub name: String,
    // thread: Thread,
    pub path: String //Must be unique
}

// impl ActorRef {
//     fn send(message: T) {
//         if mail_box.is_empty {

//         }
//     }
// }