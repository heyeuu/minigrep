use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args.get(1).expect("parse the query failed").clone();
        let file_path = args.get(2).expect("parse the file path failed").clone();
        Ok(Config { query, file_path })
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

        for line in Self::search("heyeuuu", &contents) {
            println!("{line}");
        }

        Ok(())
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
        assert_eq!(vec!["rust"], Config::search(query, contents));
    }
}
