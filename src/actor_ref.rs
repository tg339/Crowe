use actor_system::ActorSystem;
use threadpool::ThreadPool;
use rustc_serialize::json::Json;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use actor::Role;
use std::sync::mpsc::{TryRecvError, RecvError};
use std::sync::Arc;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
/// Actor Reference
/// Has in its guts the Actor(A), Message(M) and Result(R)
///
///
///


#[derive(Clone)]
struct Context<'sys, 'b: 'sys> {
    system: &'sys ActorSystem<'sys, 'b>
}


#[derive(Clone)]
pub struct ActorRef<'sys, 'b:'sys> {
    role: Arc<Box<Role + Sync + Send + 'static>>,
    context: Context<'sys, 'b>
}


impl<'sys, 'b>ActorRef<'sys, 'b> {
    pub fn new(system: &'sys ActorSystem<'sys, 'b>, 
               role: Arc<Box<Role + Sync + Send + 'static>>) -> ActorRef<'sys, 'b> {
        // Add reference to threadpool and receive function in 
        // the contructor
        ActorRef {
            role: role,
            context: Context{system: system}
        }
    }

    pub fn send(&self, message: Json) -> Receiver<String> {

        // Use the references to the receive function and pool to be able to
        // abstract away the need for the user to pass in pool and receive

        let (tx, rx) = channel();

        let role = self.role.clone();

        self.context.system.pool.execute(move || {
            let response = role.clone().receive(message);
            // print!("{:?}", );
            // let t_role = role.clone();
            tx.send(response.to_string()).unwrap();
        });

        // Recv blocks the thread until the other thread has finished
        return rx;

    }

    pub fn send_to(&self, actor_name: String, message: Json) -> Receiver<String> {

        // Use the references to the receive function and pool to be able to
        // abstract away the need for the user to pass in pool and receive
        let actor = self.context.system.actor_refs.borrow()
                        .get(&actor_name.clone())
                        .unwrap().clone();


        let (tx, rx) = channel();

        let role = actor.role.clone();

        self.context.system.pool.execute(move || {
            let response = role.clone().receive(message);
            // print!("{:?}", );
            // let t_role = role.clone();
            tx.send(response.to_string()).unwrap();
        });

        // Recv blocks the thread until the other thread has finished
        return rx;

    }
}