use std::fs::File;
use std::io::prelude::*;
use std::error::Error;


pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    //Read a file now
    let mut myfile = File::open(config.filename)?;

    let mut contents = String::new();

    myfile
        .read_to_string(&mut contents)?;

    println!("We got the following:\n{}", contents);
    Ok(())
}
