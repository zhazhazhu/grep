use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let contents = read_file(&config.filename);
    println!("With text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        return Config { query, filename };
    }
}

fn read_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}
