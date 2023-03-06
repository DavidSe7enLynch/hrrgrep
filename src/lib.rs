use std::error::Error;
use std::fs;

use log::info;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file)?;
    for line in search(&config.query, &content) {
        info!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file: args[2].to_string(),
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "32";
        let content = "\
abcd
ef
 few32 qw
g";
        println!("{}", content);
        assert_eq!(vec![" few32 qw"], search(query, content));
    }
}
