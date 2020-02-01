use crate::encode::JsonError;
use std::{fmt, result};

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

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::new(err.to_string())
    }
}

impl From<futures::channel::mpsc::SendError> for Error {
    fn from(err: futures::channel::mpsc::SendError) -> Self {
        Error::new(err.to_string())
    }
}

impl From<tktungstenite::tungstenite::Error> for Error {
    fn from(err: tktungstenite::tungstenite::Error) -> Self {
        Error::new(err.to_string())
    }
}

impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::new(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::new(err.to_string())
    }
}