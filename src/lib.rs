pub mod error;
pub mod io;
pub mod printer;

pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Line {
    line: String
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.line)
    }
}
