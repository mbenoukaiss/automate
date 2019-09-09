use std::{fmt, result};
use std::error::Error;
use crate::json::JsonError;

/// Represents an error that occurred while using
/// Automatea library.
pub struct AutomateaError {
    pub msg: String,
}

impl AutomateaError {
    pub fn new<S>(msg: S) -> AutomateaError where S: Into<String> {
        AutomateaError { msg: msg.into() }
    }

    pub fn err<S, T>(msg: S) -> Result<T, AutomateaError> where S: Into<String> {
        Err(AutomateaError { msg: msg.into() })
    }
}

impl fmt::Display for AutomateaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.msg)
    }
}

impl fmt::Debug for AutomateaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{{ msg: {} }}", self.msg)
    }
}

impl From<JsonError> for AutomateaError {
    fn from(err: JsonError) -> Self {
        AutomateaError::new(err.msg)
    }
}

impl From<ws::Error> for AutomateaError {
    fn from(err: ws::Error) -> Self {
        AutomateaError::new(err.details)
    }
}

impl From<reqwest::Error> for AutomateaError {
    fn from(err: reqwest::Error) -> Self {
        AutomateaError::new(err.description())
    }
}

impl From<fern::InitError> for AutomateaError {
    fn from(err: fern::InitError) -> Self {
        AutomateaError::new(err.description())
    }
}

impl From<log::SetLoggerError> for AutomateaError {
    fn from(err: log::SetLoggerError) -> Self {
        AutomateaError::new(err.description())
    }
}