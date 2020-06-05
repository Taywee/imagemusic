use nom::error::ParseError;
use std::num::ParseIntError;

pub enum ErrorKind {
    NomError(nom::error::ErrorKind),
    ParseIntError(ParseIntError),
}

pub struct Error<I>(I, ErrorKind);

impl<I> From<(I, nom::error::ErrorKind)> for Error<I> {
    fn from(error: (I, nom::error::ErrorKind)) -> Error<I> {
        Error(error.0, ErrorKind::NomError(error.1))
    }
}

impl<I> From<(I, ParseIntError)> for Error<I> {
    fn from(error: (I, ParseIntError)) -> Error<I> {
        Error(error.0, ErrorKind::ParseIntError(error.1))
    }
}

impl<I> ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        (input, kind).into()
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub type IResult<I, O> = nom::IResult<I, O, Error<I>>;
