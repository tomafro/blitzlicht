pub mod error;
pub mod io;
pub mod printer;

pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use regex::{ Regex, Captures };
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Line<'a> {
    line: &'a String,
    captures: Option<regex::Captures<'a>>
}

impl<'a> Line<'a> {
    pub fn new(line: &'a String) -> Line<'a> {
        lazy_static! {
            static ref SPLITTER: Regex =
                Regex::new(r"^\[(?P<context>[^\]]+)\](?: \[[^\]]+\])? \[(?P<id>[a-z0-9]+…|[a-f0-9-]+)\]").unwrap();
        }

        Line { line, captures: SPLITTER.captures(line) }
    }

    pub fn id(&self) -> &'a str {
        match &self.captures {
            Some(c) => c.name("id").unwrap().as_str(),
            None => ""
        }
    }

    pub fn context(&self) -> &'a str {
        match &self.captures {
            Some(c) => c.name("context").unwrap().as_str(),
            None => ""
        }
    }
}

impl<'a> std::fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}", self.context(), self.line)
    }
}
