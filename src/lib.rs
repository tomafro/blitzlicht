pub mod error;
pub mod io;
pub mod printer;

pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use regex::Regex;
use lazy_static::lazy_static;

pub struct Line {
    line: String,
    id: Option<String>
}

impl Line {
    pub fn new(line: String) -> Line {
        lazy_static! {
            static ref SPLITTER: Regex =
                Regex::new(r"^\[([^\]]+)\](?: \[[^\]]+\])? \[([a-z0-9]+…|[a-f0-9-]+)\]").unwrap();
        }

        if let Some(captures) = SPLITTER.captures(&line) {
            let id = Some(captures.get(2).unwrap().as_str().to_string());
            Line { line, id }
        }
        else {
            Line { line, id: None }
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {}", self.id, self.line)
    }
}
