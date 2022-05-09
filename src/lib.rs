use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use envmnt;


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = envmnt::is_or("CASE_SENSITIVE", false);

        Ok(Config { query, filename, case_sensitive})
    }
}

pub fn search<'mlifetime>(query: &str, contents: &'mlifetime str) -> Vec<&'mlifetime str>{
    let mut foundlines = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            foundlines.push(line);
        }
    }
    foundlines
}

pub fn search_insensitive<'mlifetime>(query: &str, contents: &'mlifetime str) -> Vec<&'mlifetime str>{
    let mut foundlines = Vec::new();
    let query = query.to_lowercase(); //to_lowercase

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            foundlines.push(line);
        }
    }
    foundlines
}


pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    //Read a file now
    let mut myfile = File::open(config.filename)?;

    let mut contents = String::new();

    myfile
        .read_to_string(&mut contents)?;

    let myres = if config.case_sensitive {
        search(&config.query,&contents)
    } else {
        search_insensitive(&config.query,&contents)
    };


    for line in myres{
        println!("{}",line);
    }


    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

#[test]
    fn result_one(){
        let query = "duck";
        //interesting, this one backlash is to make sure the contents do not start with an empty line
        //from https://github.com/rust-lang/regex/issues/107
        //Line-break characters are allowed in string literals. Normally they represent themselves (i.e. no translation), but as a special exception, when a U+005C character \ occurs immediately before the newline, the U+005C character, the newline, and all whitespace at the beginning of the next line are ignored. Thus a and b are equal:
        let contents = "\
Bloody hell,
living duck,
road of vice,
also Duck";

        assert_eq!(
            vec!["living duck,"],
            search(query,contents)
        );
    }

    #[test]
    fn result_insensitive(){
        let query = "duck";
        let contents = "
Bloody hell,
living Duck,
road of vice,
also duck";

        assert_eq!(
            vec!["living Duck,","also duck"],
            search_insensitive(query,contents)
        );
    }

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
