/*! This module contains or re-exports all error types for this crate.
*/

pub use crate::base32::error as base32;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::io;
use std::str;

/// An error in loading data from an ascii song
#[derive(Debug)]
pub enum AsciiLoadError {
    Format(String),
    FromBase32(base32::FromBase32Error),
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
            AsciiLoadError::FromBase32(e) => write!(f, "Error with base32: {}", e),
            AsciiLoadError::Utf8(e) => write!(f, "Error with utf8: {}", e),
            AsciiLoadError::IO(e) => write!(f, "Error with io: {}", e),
        }
    }
}

/// An error in loading data from an ascii song
#[derive(Debug)]
pub enum MusicXMLLoadError {
    Format(String),
    DomError(minidom::error::Error),
}

impl From<&str> for MusicXMLLoadError {
    fn from(description: &str) -> Self {
        MusicXMLLoadError::Format(String::from(description))
    }
}

impl From<String> for MusicXMLLoadError {
    fn from(description: String) -> Self {
        MusicXMLLoadError::Format(description)
    }
}

impl From<minidom::error::Error> for MusicXMLLoadError {
    fn from(error: minidom::error::Error) -> Self {
        MusicXMLLoadError::DomError(error)
    }
}

impl Error for MusicXMLLoadError {}

impl fmt::Display for MusicXMLLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MusicXMLLoadError::Format(s) => write!(f, "Error with input format: {}", s),
            MusicXMLLoadError::DomError(e) => write!(f, "Error with dom: {}", e),
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
    MusicXML(MusicXMLLoadError),
    GenerateSamples(GenerateSamplesError),
}

impl From<AsciiLoadError> for LoadError {
    fn from(error: AsciiLoadError) -> Self {
        LoadError::Ascii(error)
    }
}

impl From<MusicXMLLoadError> for LoadError {
    fn from(error: MusicXMLLoadError) -> Self {
        LoadError::MusicXML(error)
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
            LoadError::MusicXML(e) => write!(f, "Error with musicxml: {}", e),
            LoadError::GenerateSamples(e) => write!(f, "Error generating samples: {}", e),
        }
    }
}
