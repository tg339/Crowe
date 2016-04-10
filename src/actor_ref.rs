use actor_system::ActorSystem;
use rustc_serialize::json::Json;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::RecvError;
use actor::Role;
use std::sync::Arc;
use rand::random;
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

        // Simulate failure
        if random() {
            println!("Success send !");
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
        } else {
            println!("! Send failed :(");
            let (tx, rx) = channel();

            self.context.system.pool.execute(move || {
                panic!("Thread panicked ");
            });
            // Recv blocks the thread until the other thread has finished
            return rx;
        }
    }

    pub fn send_to(&self, actor_name: String, message: Json) -> Receiver<String> {

        // Use the references to the receive function and pool to be able to
        // abstract away the need for the user to pass in pool and receive
        let mut actor = self.context.system.actor_refs.borrow()
                        .get(&actor_name.clone())
                        .unwrap().clone();

        return actor.send(message);
    }

    pub fn safe_receive(&mut self, recv: &Receiver<String>) -> Result<String, RecvError> {
        // Try to relaunch 20 times
        return self.receive(recv, 20);
    }

    fn receive(&mut self, recv: &Receiver<String>, attempt_left: i16) -> Result<String, RecvError> {

        let result = recv.recv();

        if result.is_ok() {
            println!("Receive Success with {} attempts left", attempt_left);
            return result;
        }

        if attempt_left > 0 {
            println!("Try to relaunch the job ... ");
            let to_send = self.last_message.clone();
            let new_receiver = self.send(to_send);
            return self.receive(&new_receiver, attempt_left - 1);
        } else {
            println!("Too many failures !");
            return result
        }
    }
}
