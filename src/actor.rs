//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.

#[derive(Debug)]
struct Actor {
    name: String
}

impl Actor {
    /// This method takes a name and instantiates an actor
    ///
    fn new(name: String) -> Actor {
        Actor {
            name: name
        }
    }
}

/// This trait must be 
trait CanReceive {
    fn receive(message: Serializable);
}

impl CanReceive for Actor {
    fn receive(message: Decodable) {
        println!("{:?}", "message received");
    }
}

