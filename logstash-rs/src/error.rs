use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    FmtError(fmt::Error),
    JSONError(serde_json::Error),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(err) => write!(f, "IOError: {}", err),
            Error::FmtError(err) => write!(f, "FmtError: {}", err),
            Error::JSONError(err) => write!(f, "JSONError: {}", err),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl StdError for Error {}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::FmtError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JSONError(err)
    }
}
