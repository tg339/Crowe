// use std::collections::LinkedList;
// use std::thread::JoinHandle;

// #[derive(Debug)]
// struct EventStream {
//     events: LinkedList,
//     subscribers: vec![ActorRefs]
// }

// #[derive(Debug)]
// struct Event <T: Decodable>{
//     destinations: Vec<ActorRef>,
//     message: T
// }

// impl EventStream {
//     fn subscribe(&self, actor: ActorRef) {
//         &self.subscribers.push(ActorRef)
//         // Create the channel
//     }

//     fn publish(event: Event) {
//         for e in event.destinations {
//             // Send data through the channel
//         }
//     }
// }   