use env_logger::Builder;
use log::{error, info};
use std::{env, fs, process};

fn main() {
    Builder::new().parse_filters("debug").init();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        error!("parse args: {}", err);
        process::exit(1);
    });

    info!("searching for {} in file {}", config.query, config.file);

    let content = fs::read_to_string(config.file).expect("read file fail");
}

struct Config {
    query: String,
    file: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file: args[2].to_string(),
        })
    }
}
