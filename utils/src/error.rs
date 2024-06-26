use std::fmt;
use thiserror::Error;

/// Result alias
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for this library.
#[derive(Error, Debug)]
pub struct Error {
    pub msg: String,
    source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

// Implement the Display trait for our Error type.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

// Implement Default for Error
impl Default for Error {
    fn default() -> Self {
        Error {
            msg: "".to_string(),
            source: None,
        }
    }
}

impl Error {
    /// Create a new Error instance.
    pub fn new(msg: &str) -> Self {
        Error {
            msg: msg.to_string(),
            source: None,
        }
    }
    /// Create a new Error instance with a source error.
    pub fn with_source(msg: &str, source: Box<dyn std::error::Error + Send + Sync>) -> Self {
        Error {
            msg: msg.to_string(),
            source: Some(source),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            msg: String::from("IO Error"),
            source: Some(Box::new(err)),
        }
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Error {
            msg: String::from("Clap Error"),
            source: Some(Box::new(err)),
        }
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(err: log::SetLoggerError) -> Self {
        Error {
            msg: String::from("Logger Error"),
            source: Some(Box::new(err)),
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(_err: std::sync::PoisonError<T>) -> Self {
        Error {
            msg: String::from("Poison Error"),
            source: None,
        }
    }
}
