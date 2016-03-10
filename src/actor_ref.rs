use actor_system::ActorSystem;
use threadpool::ThreadPool;
use rustc_serialize::json::Json;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use actor::Role;
use std::sync::mpsc::{TryRecvError, RecvError};
use std::sync::Arc;
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///


// ################################
// Make a trait object that returns the
// receive function
// 
// 
//   
//    
//      
//
// ################################







#[derive(Clone)]
pub struct ActorRef<'a, A> {
    pub actor: A,
    pool: &'a ThreadPool,
    role: Arc<Box<Role + Sync + Send + 'static>>
}


impl <'a, A: Sync>ActorRef<'a, A> {
    pub fn new(actor: A, pool: &'a ThreadPool, role: Arc<Box<Role + Sync + Send + 'static>>) -> ActorRef<'a, A> {
        // Add reference to threadpool and receive function in 
        // the contructor
        ActorRef {
            actor: actor,
            pool: pool,
            role: role
        }
    }

    pub fn send(&self, message: Json) -> Receiver<String> {

        // Use the references to the receive function and pool to be able to
        // abstract away the need for the user to pass in pool and receive

        let (tx, rx) = channel();

        let role = self.role.clone();

        self.pool.execute(move || {
            role.clone().receive(message);
            // print!("{:?}", );
            // let t_role = role.clone();
            tx.send("Finished".to_string()).unwrap();
        });

        // Recv blocks the thread until the other thread has finished
        return rx;

    }
}