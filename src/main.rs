use std::env;
use std::process;


use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}",args);
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("problem in config creation: {}", err);
        process::exit(1);
    });

    println!("We are looking for {}", config.query);
    println!("in a {}", config.filename);

    if let Err(e) = minigrep::run(config){
        println!("We have a following problem, Sir! {}", e);
        process::exit(1);
    }
}
