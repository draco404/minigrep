use std::error::Error;
use std::fs;
use std::env;
use clap::{arg, command, ArgAction};

use regex::RegexBuilder;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;

    let results = search(&config.query, &contents, config.ignore_case);

    for line in results {
        println!("{}", line);
        match_regex(line, &config.query);
    }
    Ok(())
}

pub fn match_regex(text: &str, query: &str) {
    let re = RegexBuilder::new(query)
        .case_insensitive(true).build().unwrap();
    
    let matched = re.find(text).unwrap();

    println!("{}, {}", matched.start(), matched.end());
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let matches = command!()
            .version("1.0")
            .author("Alejandro V. <alejandrovieyraa7@gmail.com>") // requires `cargo` feature
            .arg(arg!([query] "Word or regex to find").required(true))
            .arg(arg!([file_path] "File path or folder path").required(false))
            .arg(
                arg!([ignore_sensitive] "Ignore case sensitive")
                    .required(false)
                    .short('i')
                    .action(ArgAction::SetTrue)
            )
            .get_matches();

        let query = match matches.get_one::<String>("query") {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        }.to_string();
        let file_path = match matches.get_one::<String>("file_path") {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        }.to_string();

        let ignore_case = matches.get_flag("ignore_sensitive");
        Ok(Config { query, file_path, ignore_case })
    } 
}

pub fn search<'a>(query: &str, contents: &'a str, sensitive: bool) -> Vec<&'a str> {
    // let re = Regex::new(&query).unwrap();
    let re = RegexBuilder::new(query)
        .case_insensitive(sensitive).build().unwrap();

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
        "To an admiring bog!"], search(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }

}