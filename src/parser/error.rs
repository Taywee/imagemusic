use nom::error::ParseError;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum Error<I> {
    NomError(I, nom::error::ErrorKind),
    ParseIntError(ParseIntError),
}

impl<I> From<(I, nom::error::ErrorKind)> for Error<I> {
    fn from(error: (I, nom::error::ErrorKind)) -> Error<I> {
        Error::NomError(error.0, error.1)
    }
}

impl<I> From<ParseIntError> for Error<I> {
    fn from(error: ParseIntError) -> Error<I> {
        Error::ParseIntError(error)
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
