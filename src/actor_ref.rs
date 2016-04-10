use actor_system::ActorSystem;
use rustc_serialize::json::Json;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use actor::Role;
use std::sync::Arc;
use std::thread::sleep;
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
    context: Context<'sys, 'b>,
    last_message: Json
}


impl<'sys, 'b>ActorRef<'sys, 'b> {
    pub fn new(system: &'sys ActorSystem<'sys, 'b>,
               role: Arc<Box<Role + Sync + Send + 'static>>) -> ActorRef<'sys, 'b> {

        ActorRef {
            role: role,
            context: Context{system: system},
            last_message: Json::from_str("{}").unwrap()
        }
    }

    pub fn send(&mut self, message: Json) -> Receiver<String> {

        // Save last message sent in case of failure
        self.last_message = message.clone();

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
        let mut actor = self.context.system.actor_refs.borrow()
                        .get(&actor_name.clone())
                        .unwrap().clone();

        return actor.send(message);
    }

    pub fn safe_receive<T>(&mut self, recv: &Receiver<T>, timeout: Duration) -> Result<T, TryRecvError> {
        return self.receive(recv, timeout, 5);
    }

    fn receive<T>(&mut self, recv: &Receiver<T>, timeout: Duration, attempt_left: i16) -> Result<T, TryRecvError> {
        // Right now we are going to try 5 times in the timeout interval given by the user
        let nb_intervals = 10;
        let timeout_interval = timeout / nb_intervals;
        for try in 0..nb_intervals {
            let result = recv.try_recv();
            if result.is_ok() {
                return result;
            }
            //println!("Could not get the result on try {} , let's try again", try);
            if attempt_left == 0 && try == nb_intervals - 1 {
                return result;
            }

            sleep(timeout_interval);
        }

        let to_send = self.last_message.clone();
        self.send(to_send);
        return self.receive(recv, timeout, attempt_left - 1);

    }
}
