pub mod error;
pub mod io;
pub mod matcher;
pub mod printer;
pub mod runner;


pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

use regex::{ Regex };
use lazy_static::lazy_static;

#[derive(Debug)]
pub struct Line<'a> {
    pub line: &'a String,
    pub id: &'a str,
    pub context: &'a str,
    pub rest: &'a str
}

impl<'a> Line<'a> {
    pub fn parse(line: &'a String) -> Option<Line<'a>> {
        lazy_static! {
            static ref SPLITTER: Regex =
                Regex::new(r"^\[(?P<context>[^\]]+)\](?: \[[^\]]+\])? \[(?P<id>[a-z0-9]+…|[a-f0-9-]+)\] (?P<rest>.*)").unwrap();
        }

        match SPLITTER.captures(line) {
            Some(captures) => {
                let id = captures.name("id").unwrap().as_str();
                let context = captures.name("context").unwrap().as_str();
                let rest = captures.name("rest").unwrap().as_str();
                Some(Line { line, id, context, rest })
            }
            None => None
        }
    }
}

impl<'a> std::fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}] [{}] {}\n", self.context, self.id, self.rest)
    }
}
