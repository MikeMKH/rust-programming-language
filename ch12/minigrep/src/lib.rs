use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_two_arguments() {
        let result = Config::new(&[
            String::from("app"),
            String::from("hello"),
            String::from("file.txt"),
        ]);
        assert_eq!(
            result.unwrap(),
            Config {
                query: String::from("hello"),
                filename: String::from("file.txt")
            }
        );
    }

    #[test]
    fn search_one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, content)
        );
    }
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("wrong number of arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Config) -> bool {
        self.query == other.query && self.filename == self.filename
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|line| {
        line.contains(query)
    }).collect()
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
