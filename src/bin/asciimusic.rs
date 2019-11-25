extern crate asciimusic;

use asciimusic::{Song, Note, Voice, Sawtooth, Sine};
use std::io::prelude::*;
use std::io;

fn main() {
    let mut song = Song {
        // quarter notes per minute -> quarter notes per second -> sixteeth notes per second
        bps: 115.0 / 60.0 * 4.0,
        // Scientific pitch F5
        base_frequency: 683.438,
        sample_rate: 48_000.0,
        voice: Voice {
            instrument: Box::new(Sawtooth{}),
            notes: vec![
                Note{
                    pitch: 0,
                    // 0 is 1 beat, because 0-beats don't make sense (maybe use in future for volume or
                    // something).  An eight note is 1 here, because 16th notes are our base.
                    length: 1,
                },
                Note{
                    // C
                    pitch: -5,
                    length: 1,
                },
                Note{
                    // Ab
                    pitch: -4,
                    length: 1,
                },
                Note{
                    // F
                    pitch: -3,
                    length: 1,
                },
                Note{
                    // Rest
                    pitch: -16,
                    length: 0,
                },
                Note{
                    // B
                    pitch: 6,
                    length: 0,
                },
                Note{
                    // Bb
                    pitch: -1,
                    length: 0,
                },
                Note{
                    // Rest
                    pitch: -16,
                    length: 0,
                },
                Note{
                    // Bb
                    pitch: 0,
                    length: 0,
                },
                Note{
                    // Ab
                    pitch: -2,
                    length: 0,
                },
                Note{
                    // Eb
                    pitch: -5,
                    length: 0,
                },
                Note{
                    // F
                    pitch: 2,
                    length: 0,
                },
                ],
        }
    };

    let bytes: Vec<u8> = song.render().iter().flat_map(|f| f.to_bits().to_be_bytes().to_vec()).collect();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&bytes).unwrap();
}
