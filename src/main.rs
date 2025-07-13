use ::std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments:{e}");
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("file path:{}", config.file_path);

    println!("In file:{}", config.file_path);

    if let Err(e) = Config::run(config) {
        println!("Application error:{e}");
        process::exit(1);
    }
}
