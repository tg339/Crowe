//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.

#[derive(Debug)]
struct ActorRef {
    name: String,
    path: String //Must be unique
}