use actor_system::ActorSystem;
use threadpool::ThreadPool;
use rustc_serialize::json::Json;
use std::sync::mpsc::channel;
use actor::Role;
use std::sync::mpsc::{TryRecvError, RecvError};
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///

#[derive(Clone)]
pub struct ActorRef<'a, A> {
    pub actor: A,
    pool: &'a ThreadPool
}


impl <'a, A>ActorRef<'a, A> {
    pub fn new(actor: A, pool: &'a ThreadPool) -> ActorRef<A> {
        // Add reference to threadpool and receive function in 
        // the contructor
        ActorRef {
            actor: actor,
            pool: pool
        }
    }

    pub fn send<F>(&self, receive: F, message: Json) -> Result<String, RecvError>
        where F: Fn(Json) + Send + 'static {

        // Use the references to the receive function and pool to be able to
        // abstract away the need for the user to pass in pool and receive

        let (tx, rx) = channel();

        // let actor = &self.receive;

        self.pool.execute(move || {
            (receive)(message);
            tx.send("Finished".to_string()).unwrap();
        });

        // Recv blocks the thread until the other thread has finished
        return rx.recv();

    }
}