use threadpool::ThreadPool;
use actor::Role;
use actor_ref::ActorRef;
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

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
/// # Example
///
/// ```
///
/// let system = ActorSystem::new(num_cpus::get());
///
/// ```
pub struct ActorSystem<'a, A: Clone> {
    // We can alternatively store actors in hashes so that they can be 
    // accessed by name. Depending on how actors are referenced this
    // could be a more efficient way of referencing actors
    pub pool: ThreadPool,
    pub actor_refs: Rc<RefCell<HashMap<String, ActorRef<'a, A>>>>
}


impl <'a, A: Clone>ActorSystem<'a, A> {
    pub fn new(thread_count: usize) -> ActorSystem<'a, A> {
        ActorSystem {
            pool: ThreadPool::new(thread_count),
            actor_refs: Rc::new(RefCell::new(HashMap::<String, ActorRef<'a, A>>::new()))
        }
    }

    pub fn spawn_actor(&'a self, name: String, actor: A) -> ActorRef<A> {
        
        let actor_ref = ActorRef::new(actor, &self.pool);

        {
            let mut actor_refs = self.actor_refs.borrow_mut();
            actor_refs.insert(name.clone(), actor_ref);    
        }

        let actor_refs = self.actor_refs.borrow().get(&name.clone()).unwrap().clone();
        
        return actor_refs;
    }
}
