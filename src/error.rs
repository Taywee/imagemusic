/*! This module contains or re-exports all error types for this crate.
*/

pub use crate::base32::error as base32;
use std::convert::From;
use std::error::Error;
use std::fmt;

/// An error in loading data, typically a song.
#[derive(Debug)]
pub enum LoadError {
    Format(String),
    FromBase32(base32::FromBase32Error),
}

impl From<&str> for LoadError {
    fn from(description: &str) -> Self {
        LoadError::Format(String::from(description))
    }
}

impl From<String> for LoadError {
    fn from(description: String) -> Self {
        LoadError::Format(description)
    }
}

impl From<base32::FromBase32Error> for LoadError {
    fn from(error: base32::FromBase32Error) -> Self {
        LoadError::FromBase32(error)
    }
}

impl Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Format(s) => write!(f, "Error with input format: {}", s),
            LoadError::FromBase32(e) => write!(f, "Error with base32: {}", e),
        }
    }
}
