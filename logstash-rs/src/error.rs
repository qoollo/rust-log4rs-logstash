use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(err) => write!(f, "IOError: {}", err),
            Error::Unknown => write!(f, "Unknown"),
        }
    }
}

impl StdError for Error {}
