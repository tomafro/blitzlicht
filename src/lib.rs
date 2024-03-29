pub mod cli;
pub mod data;
pub mod io;
pub mod matcher;
pub mod printer;
pub mod runner;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
// const REPOSITORY: &'static str = env!("CARGO_PKG_REPOSITORY");

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub tail: bool,
    pub short: bool,
    pub patterns: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum Error {
    UnableToOpenFile(String),
}

pub type Result<T> = std::result::Result<T, Error>;
