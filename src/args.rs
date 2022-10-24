use clap::{arg, command, ArgAction};

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
