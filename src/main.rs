use std::env;
use env_logger::Builder;
use log::{error, info};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    println!("Hello, world!");
}
