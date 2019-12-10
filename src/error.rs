/*! This module contains or re-exports all error types for this crate.
*/

pub use crate::base32::error as base32;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::str;

/// An error in loading data from an ascii song
#[derive(Debug)]
pub enum AsciiLoadError {
    Format(String),
    FromBase32(base32::FromBase32Error),
    Utf8(str::Utf8Error),
}

impl From<&str> for AsciiLoadError {
    fn from(description: &str) -> Self {
        AsciiLoadError::Format(String::from(description))
    }
}

impl From<String> for AsciiLoadError {
    fn from(description: String) -> Self {
        AsciiLoadError::Format(description)
    }
}

impl From<base32::FromBase32Error> for AsciiLoadError {
    fn from(error: base32::FromBase32Error) -> Self {
        AsciiLoadError::FromBase32(error)
    }
}

impl From<str::Utf8Error> for AsciiLoadError {
    fn from(error: str::Utf8Error) -> Self {
        AsciiLoadError::Utf8(error)
    }
}

impl Error for AsciiLoadError {}

impl fmt::Display for AsciiLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsciiLoadError::Format(s) => write!(f, "Error with input format: {}", s),
            AsciiLoadError::FromBase32(e) => write!(f, "Error with base32: {}", e),
            AsciiLoadError::Utf8(e) => write!(f, "Error with utf8: {}", e),
        }
    }
}

/// An error in loading data from an ascii song
#[derive(Debug)]
pub enum MusicXMLLoadError {
    Format(String),
}
