mod error;
pub use error::Error;

use nom::{
  IResult,
  // see the "streaming/complete" paragraph lower for an explanation of these submodules
  bytes::complete::{take_while, take_while1},
  number::complete::float,
  character::complete::digit1,
}; 

use crate::song::Song;

fn parse_digits(input: &str) -> IResult<&str, u64, Error<&str>> {
    let (input, sample_rate) = digit1(input)?;
    let sample_rate = sample_rate.parse().map_err(move |e| nom::Err::Failure((input, e).into()))?;
    Ok((input, sample_rate))
}

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn is_newline(c: char) -> bool {
    c == '\r' || c == '\n'
}

fn parse_song(input: &str) -> IResult<&str, Song, Error<&str>> {
    let (input, _) = take_while(is_space)(input)?;
    let (input, ticks_per_second) = float(input)?;
    let (input, _) = take_while1(is_space)(input)?;
    let (input, sample_rate) = parse_digits(input)?;
    let (input, _) = take_while1(is_newline)(input)?;
    Ok((input, Song {
        ticks_per_second,
        voices: Default::default(),
        sample_rate,
    }))
}
