// // use std::any::Any;

// // pub struct ActorWrapper {
// //     pass_message: Box<Fn(Box<Any>)>,
// // }

// // pub trait Receive {
// //     fn receive(&self, message: Box<Any>);
// // }

// // impl ActorWrapper {
// //     pub fn new<A: Any + Receive>(actor: A) -> ActorWrapper {
// //         ActorWrapper {
// //             pass_message: Box::new(move |message: Box<Any>| {
// //                 actor.receive(message);
// //             })
// //         }
// //     }
    
// //     pub fn invoke(&self, message: Box<Any>) {
// //         (self.pass_message)(message);
// //     }
// // }

// // struct MyActor;

// // impl Receive for MyActor {
// //     fn receive(&self, message: Box<Any>) {
// //         println!("received");
// //     }
// // }

// // fn main () {
// //     let wrapper = ActorWrapper::new(MyActor);
// //     wrapper.invoke(Box::new(5) as Box<Any>);
// // }
// // 
// // 
// // 
// // 
// // 
// // 
// // 
// fn main() {
//     struct MyMessage {
//         content: String
//     }
    
//     impl Message for MyMessage {
//         fn content(&self) -> String {
//             return self.content.clone();
//         }
//     }
    
//     trait Message {
//         fn content(&self) -> String;
//     }
    
//     trait Actor {
//         fn receive<M>(message: M) where  M: Send + 'static + Sized + Message;
//     }
    
//     struct Russel {
//         first_name: String
//     }
    
//     impl Actor for Russel {
//         fn receive<M>(message: M) where M: Send + 'static + Sized + Message {
    
//             fn add_exclamation(content: String) -> String {
//                 return content + "!"
//             }
    
//             let content = message.content();
//             let exclamated = add_exclamation(content);
    
//             println!("{:?}", exclamated);
//         }
        
//     }
    

//     struct Joaquin {
//         last_name: String
//     }
    
    
//     enum Actors {
//         This(Russel),
//         That(Joaquin)
//     }
    
//     let mut vec = Vec::new();
    
//     vec.push(Actors::This(Russel{first_name: "Russel".to_string()}));
//     vec.push(Actors::That(Joaquin{last_name: "Russel".to_string()}));