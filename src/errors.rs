use crate::json::JsonError;
use std::{fmt, result};
use std::error::Error as StdError;

/// Represents an error that occurred while using
/// Automate library.
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

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{{ msg: {} }}", self.msg)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::new(err.msg)
    }
}

impl From<ws::Error> for Error {
    fn from(err: ws::Error) -> Self {
        Error::new(err.details)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::new(err.description())
    }
}
