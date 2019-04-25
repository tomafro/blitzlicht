pub mod error;
pub mod io;

pub use crate::error::Error;
pub type Result<T> = std::result::Result<T, Error>;
