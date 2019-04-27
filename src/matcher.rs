use regex::{ Regex };

pub struct Matcher {
    regex: Regex,
}

impl Matcher {
    pub fn new() -> Self {
        Matcher { regex: Regex::new(r"autocompletable").unwrap() }
    }

    pub fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }
}
