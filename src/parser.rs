//mod error;
//pub use error::Error;

use nom::{
  IResult,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  bytes::complete::{take_while, take_while1},
  character::complete::digit1,
  error::{ParseError, ErrorKind}, 
  AsBytes, InputLength, Slice, AsChar,
  InputTakeAtPosition,
}; 

use std::ops::RangeFrom;

use crate::song::Song;

fn parse_digits<T, E>(input: T) -> IResult<T, u64, E>
where
    T: AsBytes + InputLength + Slice<RangeFrom<usize>> + InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
    E: ParseError<T>,
{
    let (input, sample_rate) = digit1(input)?;
    let sample_rate = match std::str::from_utf8(sample_rate.as_bytes()).unwrap().parse() {
        Ok(rate) => rate,
        // TODO: use _e for context
        Err(_e) => return Err(nom::Err::Failure(E::from_error_kind(input, ErrorKind::Digit))),
    };
    Ok((input, sample_rate))
}

fn is_space<T: AsChar>(c: T) -> bool {
    let c = c.as_char();
    c == ' ' || c == '\t'
}

fn is_newline<T: AsChar>(c: T) -> bool {
    let c = c.as_char();
    c == '\r' || c == '\n'
}

fn is_whitespace<T: AsChar + Copy>(c: T) -> bool {
    is_space(c) || is_newline(c)
}

pub fn parse_song<T, E>(input: T) -> IResult<T, Song, E> 
where 
    T: AsBytes + InputLength + Slice<RangeFrom<usize>> + InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar + Copy,
    E: ParseError<T>,
{
    let (input, _) = take_while(is_whitespace)(input)?;
    let (input, ticks_per_second) = parse_digits(input)?;
    let (input, _) = take_while1(is_space)(input)?;
    let (input, sample_rate) = parse_digits(input)?;
    let (input, _) = take_while1(is_newline)(input)?;
    Ok((input, Song {
        ticks_per_second,
        voices: Default::default(),
        sample_rate,
    }))
}
