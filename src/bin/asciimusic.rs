extern crate asciimusic;

use asciimusic::parser::parse_song;
use asciimusic::Song;
use std::io;
use std::io::prelude::*;
use nom::IResult;
use nom::error::VerboseError;
use nom::error::convert_error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = r#"
    666666666666666666666666666666666666666666666666666666666666666 48000
    1 0,0/1,0.005/0.75,0.01/0,-0.01 1 512 UA QA AA QA AA MA UB TB AB EB AB
    "#;

    let result: IResult<&str, Song, VerboseError<&str>> = parse_song(input);
    let (_, song) = match result {
        Ok(value) => value,
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
            let value = convert_error(input, e);
            eprintln!("{}", value);
            panic!("dying now");
        }
        Err(e) => panic!("{}", e),
    };
    println!("{:#?}", song);
    Ok(())
}
