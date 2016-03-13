use threadpool::ThreadPool;
use actor::Role;
use actor_ref::ActorRef;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;


/// A central actor system which manages the actor references and actors
///
/// Spawns the actor system with a specified number of threads in the 
/// central thread pool. We suggest you spin up the same number of cores
/// that you have in your system.
///
/// You can use the num_cpus crate to estimate the number of cores on your 
/// system
///
///
///
pub struct ActorSystem<'sys, 'b: 'sys> {
    // We can alternatively store actors in hashes so that they can be 
    // accessed by name. Depending on how actors are referenced this
    // could be a more efficient way of referencing actors
    pub pool: ThreadPool,
    pub actor_refs: Rc<RefCell<HashMap<String, ActorRef<'sys, 'b>>>>
    // pub actors: Rc<RefCell<HashMap<Stringrc<Box<Role + Send + 'static>>>>>
}


impl <'sys, 'b>ActorSystem<'sys, 'b> {
    pub fn new(thread_count: usize) -> ActorSystem<'sys, 'b> {
        ActorSystem {
            pool: ThreadPool::new(thread_count),
            actor_refs: Rc::new(RefCell::new(HashMap::<String, ActorRef<'sys, 'b>>::new())),
        }
    }

    pub fn spawn_actor(&'sys self, name: String, role: Box<Role + Sync + Send + 'static>) -> ActorRef<'sys, 'b> {
        
        let arc_role = Arc::new(role);

        let actor_ref = ActorRef::new(&self, arc_role.clone());
        // let actor_ref = ActorRef::new(&self.pool, arc_role.clone());

        {
            let mut actor_refs = self.actor_refs.borrow_mut();
            actor_refs.insert(name.clone(), actor_ref.clone()); 

        }

        let actor_refs = self.actor_refs.borrow().get(&name.clone()).unwrap().clone();
        
        return actor_refs;
        
    }
}
