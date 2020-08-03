use std::{fmt, result};
use crate::encode::json::JsonError;

/// Represents an error that occurred while using
/// Automate library.
#[derive(Debug)]
pub struct Error {
    pub msg: String,
}

impl Error {
    pub fn new<S>(msg: S) -> Error where S: Into<String> {
        Error { msg: msg.into() }
    }

    pub fn err<S, T>(msg: S) -> Result<T, Error> where S: Into<String> {
        Err(Error { msg: msg.into() })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::new(err.msg)
    }
}

impl<T: std::error::Error> From<T> for Error {
    fn from(err: T) -> Self {
        Error::new(err.to_string())
    }
}
