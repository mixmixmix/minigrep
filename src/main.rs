use std::env;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("We are looking for {}", query);
    println!("in a {}", filename);

    //Read a file now
    let mut myfile = File::open(filename).expect("File ??? not found :(");
    let mut contents = String::new();

    myfile.read_to_string(&mut contents).expect("Something went wrong when reading the contents");

    println!("We got the following:\n{}", contents);


}
