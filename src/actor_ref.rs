use std::sync::{Arc, Mutex};
use actor::Actor;
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///


#[derive(Debug)]
struct ActrorRef<A: Actor + 'static> {
    actor: Arc<Mutex<A>>
}