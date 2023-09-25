use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    let contents = read_file(&config.filename);
    println!("With text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        return Ok(Config { query, filename });
    }
}

fn read_file(filename: &String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents;
}
