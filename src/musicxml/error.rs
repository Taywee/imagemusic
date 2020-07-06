use crate::note::NoSuchNoteName;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidMusicXML(&'static str),
    BadNumber(ParseIntError),
    NoSuchNoteName,
    GraceNote,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::BadNumber(error)
    }
}

impl From<NoSuchNoteName> for Error {
    fn from(_error: NoSuchNoteName) -> Self {
        Error::NoSuchNoteName
    }
}
