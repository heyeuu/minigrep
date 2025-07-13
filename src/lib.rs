use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args.get(1).expect("parse the query failed").clone();
        let file_path = args.get(2).expect("parse the file path failed").clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }

    pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
        let mut results: Vec<&str> = Vec::new();

        for line in contents.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
        results
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.file_path)?;

        let result = if config.ignore_case {
            Self::search_case_insensitive(&config.query, &contents)
        } else {
            Self::search(&config.query, &contents)
        };

        for line in result {
            println!("{line}");
        }

        Ok(())
    }

    pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let mut result = Vec::new();
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                result.push(line);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "rust";
        let contents = "/ hello world!
rust
        hahhh I'm readyyyyy";
        assert_eq!(vec!["rust",], Config::search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "heyeuuu";
        let contents = "\
        Hahhhh
this is a test of rust
written by 
heyeuuu
instead of heyeu
and
Heyeuuu";
        assert_eq!(vec!["heyeuuu"], Config::search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
        hello~
        I am a girl who want to pursur her happiness and freedom
and I 
wanna to study
rust and prepare a surprise for
myself!
let's rusty";

        assert_eq!(
            vec!["rust and prepare a surprise for", "let's rusty"],
            Config::search_case_insensitive(query, contents)
        );
    }
}
