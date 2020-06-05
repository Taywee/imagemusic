extern crate asciimusic;

use asciimusic::parser::parse_song;
use asciimusic::Song;
use std::io;
use std::io::prelude::*;
use nom::IResult;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
    6 48000
    1 0,0/1,0.005/0.75,0.01/0,-0.01 1 512 UA QA AA QA AA MA UB TB AB EB AB
    "#;

    let result: IResult<&str, Song> = parse_song(input);
    let (_, song) = result?;
    println!("{:#?}", song);
    Ok(())
}
