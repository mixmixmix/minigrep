use std::env;
use std::fs::File;
use std::io::prelude::*;

//my interesting alternative of keeping the `query` and `filename` as string slices (as it was before we create a struct) in a book.
struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = &args[1];
        let filename = &args[2];

        Config { query, filename }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args); //so that should work always as args are existing in any case with at least a program name
    println!("Those were the args {:?}", args); //possible because we have inmutable reference in config.

    println!("We are looking for {}", config.query);
    println!("in a {}", config.filename);

    //Read a file now
    let mut myfile = File::open(config.filename).expect("File ??? not found :(");
    let mut contents = String::new();

    myfile
        .read_to_string(&mut contents)
        .expect("Something went wrong when reading the contents");

    println!("We got the following:\n{}", contents);
}
