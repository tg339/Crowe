use threadpool::ThreadPool;
use actor::Actor;
use actor_ref::ActorRef;
use std::collections::HashMap;

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
pub struct ActorSystem<A: Actor> {
    // We can alternatively store actors in hashes so that they can be 
    // accessed by name. Depending on how actors are referenced this
    // could be a more efficient way of referencing actors
    pub pool: ThreadPool,
    pub actor_refs: HashMap<String, ActorRef<A>>
}


impl <A>ActorSystem<A> where A: Actor   {
    pub fn new(thread_count: usize) -> ActorSystem<A> {
        ActorSystem {
            pool: ThreadPool::new(thread_count),
            actor_refs: HashMap::<String, ActorRef<A>>::new()
        }
    }

    pub fn spawn_actor(&mut self, actor: A) -> &ActorRef<A> {
        let actor = actor.clone();
        
        self.actor_refs.insert(actor.name(), ActorRef::new(actor.clone()));
        return self.actor_refs.get(&*actor.name()).unwrap();
    }
}
