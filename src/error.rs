use std::fmt::Display;
use std::io::Error as IoErr;

use crate::validation::ValidationErr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoErr),
    Validation(ValidationErr),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for Error {}

impl From<IoErr> for Error {
    fn from(err: IoErr) -> Self {
        Self::Io(err)
    }
}

impl From<ValidationErr> for Error {
    fn from(err: ValidationErr) -> Self {
        Self::Validation(err)
    }
}
