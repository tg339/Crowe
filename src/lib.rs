extern crate rustc_serialize;
extern crate time;

pub mod actor;
pub mod actor_system;
pub mod event_stream;
pub mod actor_ref;

// Needs to be changed to make real tests
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::add_two;

    #[test]
    fn bandwidth() {
        assert_eq!(4, add_two(2));
    }
}
