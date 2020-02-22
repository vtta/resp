use std::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    kind: ErrorKind,
}

enum ErrorKind {}

