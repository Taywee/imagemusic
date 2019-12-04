use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct FromBase32Error {
    pub source: char,
}

#[derive(Debug)]
pub struct ToBase32Error<T> where T: Sized + fmt::Debug {
    pub source: T,
}

impl Error for FromBase32Error {
}

impl fmt::Display for FromBase32Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not use {} as a base32 character", self.source)
    }
}

impl<T> Error for ToBase32Error<T> where T: fmt::Debug + fmt::Display, Self: fmt::Debug + fmt::Display {
}

impl fmt::Display for ToBase32Error<i8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not convert {} to base32. Range is [-15, 16]", self.source)
    }
}

impl fmt::Display for ToBase32Error<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not convert {} to base32. Range is [0, 31]", self.source)
    }
}

