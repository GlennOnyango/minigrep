use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Search for {}", config.query);

    println!("In {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Should be able to read file");

    println!("The contents of the file\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config{

    fn new(args: &[String]) -> Config {

        if args.len() < 3{
            panic!("Not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Config { query, file_path }
    }
}


