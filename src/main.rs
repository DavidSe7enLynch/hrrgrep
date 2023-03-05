use env_logger::Builder;
use log::{error, info};
use std::{env, fs};

fn main() {
    Builder::new().parse_filters("debug").init();

    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    info!("searching for {} in file {}", config.query, config.file);

    let content = fs::read_to_string(config.file).expect("read file fail");
}

struct Config {
    query: String,
    file: String,
}

fn parse_args(args: &Vec<String>) -> Config {
    Config {
        query: args[1].to_string(),
        file: args[2].to_string(),
    }
}
