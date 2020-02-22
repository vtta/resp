use std::{error, fmt, io, result, string};

use serde::{de, ser};

/// Alias type for Result<T, Error>
pub type Result<T> = result::Result<T, Error>;

/// Meaning of a error
#[derive(Debug)]
pub enum Error {
    /// Error that only contains error message, usually came from ser/de error
    Msg(String),
    /// Due to restriction of RESP, array len must known to be used as prefix
    LenNotKnown,
    /// Write error
    Io,
    /// Write buf cannot be represented by valid UTF-8 string
    Utf8,
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

impl From<io::Error> for Error {
    fn from(_: io::Error) -> Self {
        Error::Io
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(_: string::FromUtf8Error) -> Self {
        Error::Utf8
    }
}
