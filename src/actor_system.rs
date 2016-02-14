//! The fundamental computing unit of the Crowe actor system
//! 
//! An actor is a high level abstraction of a computing unit. The actor can receive a message.

#[derive(Debug)]
struct ActorSystem {
    address: String,
    name: String,
    event_stream: EventStream
}

impl ActorSystem {
    /// This method takes a name and address and generates a new actor.
    ///
    ///
    /// This should be refactored to take in a name and a configuration.
    /// 
    fn new(name, address) -> ActorSystem {
        ActorSystem {
            name: name,
            address: address,
            event_stream: EventStream::new()
        }
    }

    /// Create an actor and return its reference
    /// This method needs to subsribe the actor to the event stream
    ///
    fn create_actor() -> ActorRef {
        EventStream::subsribe(ActorRef);
    }


}