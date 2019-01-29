use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file name"),
        };

        Ok(Config { query, filename })
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Config) -> bool {
        self.query == other.query && self.filename == self.filename
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    for line in search(&config.query, &content) {
        println!("{}", line);
    }

    Ok(())
}
