use std::sync::{Arc, Mutex};
use actor::Actor;
use actor_system::ActorSystem;
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///


#[derive(Debug)]
pub struct ActorRef<A: Actor + Sized + 'static> {
    actor: Arc<Mutex<A>>
}


impl <A>ActorRef<A> where A: Actor + Sized + 'static {
    pub fn new(actor: A) -> ActorRef<A> {
        ActorRef {
            actor: Arc::new(Mutex::new(actor))
        }
    }

    pub fn send<F, M>(system: ActorSystem<A>, receive: F, message: M) 
                      where F: Fn(M) + Send + 'static, M: Sized + Send + 'static {
        system.pool.execute(move|| {
            (receive)(message)
        })
    }
}