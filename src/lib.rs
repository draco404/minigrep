use std::{error::Error, process};
use std::fs;
use colored::Colorize;
use arboard::Clipboard;

mod search;
pub mod args;
use search::{match_regex, search};
use args::*;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.file_path)?;
    let results = search(&config.query, &contents, config.ignore_case);
    for line in &results {
        let (start, end, text) = match_regex(line, &config.query);
        let mut line = line.to_string();
        line.replace_range(start..end, &text.red().to_string());
        println!("{}", line);
    }
    if config.copy_to_clipboard {
        let text = results.join("\n");
        let mut clipboard = Clipboard::new().unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1);
        });
        clipboard.set_text(text).unwrap_or_else(|err| eprintln!("{}", err));
    }
    Ok(())
}
