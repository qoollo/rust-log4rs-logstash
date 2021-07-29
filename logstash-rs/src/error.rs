use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Fmt(fmt::Error),
    JSON(serde_json::Error),
    Internal(String),
    Unknown,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IOError: {}", err),
            Error::Fmt(err) => write!(f, "FmtError: {}", err),
            Error::JSON(err) => write!(f, "JSONError: {}", err),
            Error::Internal(err) => write!(f, "InternalError: {}", err),
            Error::Unknown => write!(f, "Unknown error"),
        }
    }
}

impl StdError for Error {}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Self::Fmt(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JSON(err)
    }
}
