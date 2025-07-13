use ::std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = args.get(1).expect("parse the query failed");
    let file_path = args.get(2).expect("parse the file path failed");

    println!("Query: {}", query);
    println!("file path:{}", file_path);

    println!("In file:{file_path}");
    let content = fs::read_to_string(file_path).expect("failed to read file");
    println!("with content:{content}");


}
