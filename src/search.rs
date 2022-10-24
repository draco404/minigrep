use regex::RegexBuilder;

pub fn match_regex(text: &str, query: &str) -> (usize, usize, String) {
    let re = RegexBuilder::new(query)
        .case_insensitive(true).build().unwrap();
    
    let matched = re.find(text).unwrap();
    let mrange = matched.start()..matched.end();
    let original = text[mrange].to_string();

    (matched.start(), matched.end(), original)
}

pub fn search<'a>(query: &str, contents: &'a str, sensitive: bool) -> Vec<&'a str> {
    let re = RegexBuilder::new(query)
        .case_insensitive(sensitive).build().unwrap();

    contents.lines().filter(|line| re.is_match(&line)).collect()
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