mod error;
pub use error::Error;

use nom::{
  IResult,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  bytes::complete::{take_while, take_while1},
  character::complete::digit1,
}; 

use crate::song::Song;

fn parse_digits(input: &str) -> IResult<&str, u64, Error<&str>> {
    let (input, sample_rate) = digit1(input)?;
    let sample_rate = sample_rate.parse().map_err(|e| nom::Err::Failure(Error::from(e)))?;
    Ok((input, sample_rate))
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn is_newline(c: char) -> bool {
    c == '\r' || c == '\n'
}

fn is_whitespace(c: char) -> bool {
    is_space(c) || is_newline(c)
}

pub fn parse_song(input: &str) -> IResult<&str, Song, Error<&str>> {
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
