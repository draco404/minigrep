use std::error::Error;
use std::fs;
use std::env;

use regex::Regex;
use regex::RegexBuilder;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path , ignore_case })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(&query).unwrap();
    contents.lines().filter(|line| re.is_match(line)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let re = Regex::new(&query).unwrap();
    let re = RegexBuilder::new(query)
        .case_insensitive(true).build().unwrap();

    contents.lines().filter(|line| re.is_match(&line)).collect()
    // contents.lines().filter(|line| line.to_lowercase().contains(&query.to_lowercase())).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "^T";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

        assert_eq!(vec!["Then there's a pair of us - don't tell!",
        "They'd banish us, you know.",
        "To tell your name the livelong day",
        "To an admiring bog!"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }

}