pub trait Actor {
    fn receive(message: Send);
}

