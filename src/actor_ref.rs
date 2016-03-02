use std::sync::{Arc, Mutex};
use actor::Actor;
use actor_system::ActorSystem;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///


#[derive(Debug)]
pub struct ActorRef<A: Actor + Sized + 'static> {
    pub actor: Arc<Mutex<A>>
}


impl <A>ActorRef<A> where A: Actor + Sized + 'static {
    pub fn new(actor: A) -> ActorRef<A> {
        ActorRef {
            actor: Arc::new(Mutex::new(actor))
        }
    }

    pub fn send<F, M>(&self, pool: &ThreadPool, receive: F, message: M) -> String
        where F: Fn(M) + Send + 'static,
              M: Sized + Send + 'static {

        let (tx, rx) = channel();        

        pool.execute(move|| {
            (receive)(message);
            tx.send("Finished".to_string()).unwrap();
        });

        return rx.recv().unwrap();

    }
}