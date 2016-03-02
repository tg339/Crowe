use std::sync::{Arc, Mutex};
use actor::Actor;
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///


#[derive(Debug)]
pub struct ActorRef<A: Actor + Sized + 'static> {
    actor: Arc<Mutex<A>>
}