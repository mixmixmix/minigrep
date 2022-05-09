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

#[cfg(test)]
mod test {
    use super::*;

// #[test]
//     fn result_one(){
//         let query = "duck";
//         let contents = "\
// Bloody hell,
// living duck,
// road of vice!";

//         assert_eq!(
//             vec!["living duck,"],
//             search(query,contents)
//         );
//     }

    //You might ask my friend why those arrays of string are so weird. It is Rust. But also it is because we decided to use Strings, and clone them when passing arguments, instead of having &str so that we don't have to "worry" about lifetimes. In the future we'll be proper!!! ;)
    #[test]
    fn config_test(){
        assert!(Config::new(&["too".to_string(), "little".to_string()]).is_err());
    }

    #[test]
    fn run_test_good(){
        let config = Config::new(&["minigrep_target_name".to_string(),
                                   "query".to_string(),
                                   "files/haystack.txt".to_string()]).unwrap();

        run(config).unwrap();
    }

    #[test]
    fn run_test_bad(){
        let config = Config::new(&["minigrep_target_name".to_string(),
                                   "query".to_string(),
                                   "NONEXISTENT_FILE.txt".to_string()]).unwrap();
        assert!(run(config).is_err());
    }

    #[test]
    #[should_panic]
    fn run_pass_err(){
        let config = Config::new(&["minigrep_target_name".to_string()
                                   ]);
        run(config.unwrap()).unwrap(); //this little unwrap will panic because we are paasing there an unhandled error!
    }

}
