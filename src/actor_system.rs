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
/// # Example
///
/// ```
///
/// let system = ActorSystem::new(num_cpus::get());
///
/// ```
pub struct ActorSystem<'a, A: Clone + Sync> {
    // We can alternatively store actors in hashes so that they can be 
    // accessed by name. Depending on how actors are referenced this
    // could be a more efficient way of referencing actors
    pub pool: ThreadPool,
    pub actor_refs: Rc<RefCell<HashMap<String, ActorRef<'a, A>>>>
    // pub actors: Rc<RefCell<HashMap<String, Arc<Box<Role + Send + 'static>>>>>
}


impl <'a, A: Clone + Sync>ActorSystem<'a, A> {
    pub fn new(thread_count: usize) -> ActorSystem<'a, A> {
        ActorSystem {
            pool: ThreadPool::new(thread_count),
            actor_refs: Rc::new(RefCell::new(HashMap::<String, ActorRef<'a, A>>::new())),
            // actors: Rc::new(RefCell::new(HashMap::<String, Box<Role + Send + 'static>>::new()))
        }
    }

    pub fn spawn_actor(&'a self, name: String, actor: A,  role: Box<Role + Sync + Send + 'static>) -> ActorRef<'a, A> {
        
        let arc_role = Arc::new(role);

        let actor_ref = ActorRef::new(actor, &self.pool, arc_role.clone());

        {
            let mut actor_refs = self.actor_refs.borrow_mut();
            actor_refs.insert(name.clone(), actor_ref.clone()); 

        }

        // {
        //     let mut actors = self.actors.borrow_mut();
        //     actors.insert(name.clone(), role);    
        // }

        let actor_refs = self.actor_refs.borrow().get(&name.clone()).unwrap().clone();
        // let actor_refs = self.actors.borrow().get(&name.clone()).unwrap().receive("this");
        return actor_refs;
        
    }
}
