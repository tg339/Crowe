use threadpool::ThreadPool;
use actor::Actor;
use actor_ref::ActorRef;

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
/// let pool = ActorSystem::new(num_cpus::get());
///
/// ```
pub struct ActorSystem<A: Actor + Sized + 'static> {
    pool: ThreadPool,
    actors: Vec<A>,
    references: Vec<ActorRef<A>>
}

impl <A>ActorSystem<A> where A: Actor + Sized + 'static {
    fn new(thread_count: usize) -> ActorSystem<A> {
        ActorSystem {
            pool: ThreadPool::new(thread_count),
            actors: Vec::<A>::new(),
            references: Vec::<ActorRef<A>>::new(),
        }
    }

    fn spawn_actor(&mut self, actor: A) -> ActorRef<A>{
        unimplemented!()   
    }
}