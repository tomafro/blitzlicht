use regex::{ RegexSet };

pub struct Matcher {
    regex: RegexSet,
}

impl Matcher {
    pub fn new(patterns: Option<Vec<String>>) -> Self {
        let regex = match patterns {
            Some(values) => RegexSet::new(values).unwrap(),
            None => RegexSet::new(&[r"."]).unwrap()
        };
        Matcher { regex }
    }

    pub fn matches(&self, line: &str) -> bool {
        self.regex.is_match(line)
    }
}
