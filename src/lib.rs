use std::error::Error;
use std::fs;
use colored::Colorize;
pub mod args;
mod search;
use search::{match_regex, search};
use args::*;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = search(&config.query, &contents, config.ignore_case);

    for line in results {
        let (start, end, text) = match_regex(line, &config.query);
        let mut line = line.to_string();
        line.replace_range(start..end, &text.red().to_string());
        println!("{}", line);
    }
    Ok(())
}
