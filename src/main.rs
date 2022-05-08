use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;


//my interesting alternative of keeping the `query` and `filename` as string slices (as it was before we create a struct) in a book.
struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn run (config: Config) -> Result<(), Box<dyn Error>> {

    //Read a file now
    let mut myfile = File::open(config.filename)?;

    let mut contents = String::new();

    myfile
        .read_to_string(&mut contents)?;

    println!("We got the following:\n{}", contents);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("problem in config creation: {}", err);
        process::exit(1);
    });

    println!("We are looking for {}", config.query);
    println!("in a {}", config.filename);

    if let Err(e) = run(config){
        println!("We have a following problem, Sir! {}", e);
        process::exit(1);
    }
}
