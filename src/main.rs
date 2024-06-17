use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    println!("");

    let config = Config::new(&args);
    println!("-- Config:\nSearching: {}", config.query);
    println!("In file: {}\n--\n", config.file_path);

    let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");
    println!("-- Content:\n{contents}\n--");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
