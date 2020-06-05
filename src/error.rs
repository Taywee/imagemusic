/*! This module contains or re-exports all error types for this crate.
*/

use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::str;

/// An error in loading data from an ascii song
#[derive(Debug)]
pub enum AsciiLoadError {
    Format(String),
    Utf8(str::Utf8Error),
    IO(io::Error),
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

impl From<str::Utf8Error> for AsciiLoadError {
    fn from(error: str::Utf8Error) -> Self {
        AsciiLoadError::Utf8(error)
    }
}

impl From<io::Error> for AsciiLoadError {
    fn from(error: io::Error) -> Self {
        AsciiLoadError::IO(error)
    }
}

impl Error for AsciiLoadError {}

impl fmt::Display for AsciiLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AsciiLoadError::Format(s) => write!(f, "Error with input format: {}", s),
            AsciiLoadError::Utf8(e) => write!(f, "Error with utf8: {}", e),
            AsciiLoadError::IO(e) => write!(f, "Error with io: {}", e),
        }
    }
}

/// An error in loading data from an ascii song
#[derive(Debug)]
pub enum GenerateSamplesError {
    EmptyVoice,
}

impl Error for GenerateSamplesError {}

impl fmt::Display for GenerateSamplesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerateSamplesError::EmptyVoice => {
                write!(f, "Tried to generate VoiceIterator an empty voice")
            }
        }
    }
}

/**
 * Top-level load error type.
 *
 * This is a convenience type for the main function.
 */
#[derive(Debug)]
pub enum LoadError {
    Ascii(AsciiLoadError),
    GenerateSamples(GenerateSamplesError),
}

impl From<AsciiLoadError> for LoadError {
    fn from(error: AsciiLoadError) -> Self {
        LoadError::Ascii(error)
    }
}

impl From<GenerateSamplesError> for LoadError {
    fn from(error: GenerateSamplesError) -> Self {
        LoadError::GenerateSamples(error)
    }
}

impl Error for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Ascii(e) => write!(f, "Error with ascii: {}", e),
            LoadError::GenerateSamples(e) => write!(f, "Error generating samples: {}", e),
        }
    }
}
