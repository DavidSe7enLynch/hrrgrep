use std::env;
use env_logger::Builder;
use log::{error, info};

fn main() {
    Builder::new().parse_filters("debug").init();
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let query = &args[1];
    let file = &args[2];
    info!("Hello, world!");
}
