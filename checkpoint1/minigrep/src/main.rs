use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!(
        "Searching for '{}' in the file '{}'",
        config.query, config.file_path
    );

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file.");
    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &Vec<String>) -> Config {
    let query = &args[1];
    let file_path = &args[2];
    Config {
        query: query.to_string(),
        file_path: file_path.to_string(),
    }
}
