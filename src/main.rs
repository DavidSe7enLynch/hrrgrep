use env_logger::Builder;
use log::{error, info};
use std::{env, process};
use hrrgrep::Config;

fn main() {
    Builder::new().parse_filters("debug").init();

    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        error!("parse args: {e}");
        process::exit(1);
    });

    info!("searching for {} in file {}", config.query, config.file);

    if let Err(e) = hrrgrep::run(config) {
        error!("run: {e}");
        process::exit(1);
    }
}


