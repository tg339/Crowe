use std::fs::OpenOptions;
use std::fs::File;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::io::prelude::*;

pub fn create_checkpoints(num_workers: usize) {
    for i in 1..(num_workers + 1) {
        let mut file = File::create("checkpoints/checkpoint".to_string() + &*i.to_string() +  ".txt").unwrap();
    }
    let mut file = File::create("checkpoints/checkpoint_main.txt").unwrap();
}


pub fn checkpoint_message(message: Json, workerId:usize) {
    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("checkpoints/checkpoint".to_string() + &*workerId.to_string() +  ".txt")
            .unwrap();

    file.write_all( (json::encode(&message).unwrap() + "\n").as_bytes()).unwrap()
}

pub fn checkpoint_result(workerId:usize) {
    let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("checkpoints/checkpoint_main.txt")
            .unwrap();

    file.write_all(workerId.to_string().as_bytes()).unwrap()
}

pub fn read_checkpoint_message(workerId:usize) -> String {
    let mut file = OpenOptions::new()
            .read(true)
            .open("checkpoints/checkpoint".to_string() + &*workerId.to_string() +  ".txt")
            .unwrap();

    let mut s = String::new();
    let mut line = file.read_to_string(& mut s).unwrap();

    return s
}

pub fn read_checkpoint_main() -> Vec<usize>{
    let mut file = OpenOptions::new()
            .read(true)
            .open("checkpoints/checkpoint_main.txt")
            .unwrap();

    let mut s = String::new();
    let mut line = file.read_to_string(& mut s).unwrap();
    let chars = s.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();


    return chars
}
