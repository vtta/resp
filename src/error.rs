use std::{error, fmt, result};

use serde::{de, ser};

/// Alias type for Result<T, Error>
pub type Result<T> = result::Result<T, Error>;

/// Meaning of a error
#[derive(Debug)]
pub enum Error {
    /// Error that only contains error message, usually came from ser/de error
    Msg(String),
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Msg(msg.to_string())
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Msg(msg.to_string())
    }
}

impl Error {
    fn as_str(&self) -> &str {
        match self {
            _ => "other error",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
