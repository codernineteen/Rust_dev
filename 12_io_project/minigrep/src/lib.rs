use std::env;
use std::error::Error;
use std::fs; //To read file

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    //keep parsing logic in main but separate functionality.
    //env::args() returns iterator whose type is std::env::Args
    //The reason we added 'static lifetime follows Lifetime elision rule
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        //set env variable
        //we don't care about value when env was set. we only care whether it is set or not. That's why we use is_err method.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

//Here, actual code execution
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //return unit type if it is OK
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) //need return
    } else {
        search_case_insensitive(&config.query, &contents) //need return
    };

    for line in results {
        println!("{}", line);
    }

    Ok(()) // this is just an idiomatic way for run function
}

//the data returned by this function will live as long as the data passed into search function in 'contents' argument
//By appending 'a in returned references, we are telling rust we need contents arguments to return
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

//TDD process for helping to build bug-free program
//1. Make a test whose result is failure deliberately
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // "\ tells rust not to put a newline character at the beginning of the contents.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    //we need environment variable for case-insensitive.
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
