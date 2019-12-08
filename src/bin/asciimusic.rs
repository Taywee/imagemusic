extern crate asciimusic;

use asciimusic::error::LoadError;
use asciimusic::Song;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), LoadError> {
    let input = r#"
    6 48000
    # Sawtooth, Square, Sine, Triangle
    1 0,0/1,0.005/0.75,0.01/0,-0.01 1 512 UA QA AA QA AA MA UB TB AB EB AB
    "#;

    let mut song = Song::load_from_str(&input)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for sample in song.samples() {
        handle.write_all(&sample.to_bits().to_be_bytes()).unwrap();
    }
    Ok(())
}
