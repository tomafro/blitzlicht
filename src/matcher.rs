use regex::{ RegexSet };

pub struct Matcher {
    regex: Option<RegexSet>,
}

impl Matcher {
    pub fn new(patterns: Option<Vec<String>>) -> Self {
        let regex = match patterns {
            Some(values) => Some(RegexSet::new(values).unwrap()),
            None => None
        };
        Matcher { regex }
    }

    pub fn matches(&self, line: &str) -> bool {
        match &self.regex {
            Some(regex) => regex.is_match(line),
            None => true
        }
    }
}
