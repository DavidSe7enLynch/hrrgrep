use std::error::Error;
use std::{env, fs};

use log::info;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file)?;

    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in result {
        info!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or("missing arg: query")?;
        let file = args.next().ok_or("missing arg: file")?;

        Ok(Config {
            query,
            file,
            ignore_case: env::var("IGNORE_CASE").is_ok(),
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

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query_low = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query_low) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        println!("{}", content);
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        println!("{}", content);
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, content)
        );
    }
}
