use std::fmt;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Error {
    NoTargetFound,
    SuperpixelGridNotSquare,
    InvalidDimensions,
    InvalidLength { encoded: u16, available: u16 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
