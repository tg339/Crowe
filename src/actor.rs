use rustc_serialize::json::Json;

pub trait Role {
    fn receive(&self, message: Json) -> Json;
}
