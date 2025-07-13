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

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(config.file_path)?;
        println!("with content:{content}");

        Ok(())
    }
}
