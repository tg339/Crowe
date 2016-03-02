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


pub struct ActorRef<A: Actor> {
    pub actor: Arc<Mutex<A>>
}


impl <A>ActorRef<A> where A: Actor {
    pub fn new(actor: A) -> ActorRef<A> {
        // Add reference to threadpool and receive function in 
        // the contructor
        ActorRef {
            actor: Arc::new(Mutex::new(actor))
        }
    }

    pub fn send<F, M>(&self, pool: &ThreadPool, receive: F, message: M) -> String
        where F: Fn(M) + Send + 'static,
              M: Sized + Send + 'static {

        // Use the references to the receive function and pool to be able to
        // abstract away the need for the user to pass in pool and receive

        let (tx, rx) = channel();       

        pool.execute(move || {
            (receive)(message);
            tx.send("Finished".to_string()).unwrap();
        });

        // Recv blocks the thread until the other thread has finished
        return rx.recv().unwrap();

    }
}