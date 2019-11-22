extern crate asciimusic;

use asciimusic::{Song, Note};
use std::io::prelude::*;
use std::io;

fn main() {
    let song = Song {
        bps: 7.6666667,
        // Scientific pitch Eb4
        base_frequency: 311.1275,
        notes: vec![
            Note{
                // 0 is rest, 1 is Eb4, 13 is Eb5, 15 is F5
                pitch: 15,
                // 0 is 1 beat, because 0-beats don't make sense (maybe use in future for volume or
                // something).  An eight note is 1 here, because 16th notes are our base.
                length: 1,
            },
            Note{
                // C
                pitch: 10,
                length: 1,
            },
            Note{
                // Ab
                pitch: 6,
                length: 1,
            },
            Note{
                // F
                pitch: 3,
                length: 1,
            },
            Note{
                // Rest
                pitch: 0,
                length: 0,
            },
            Note{
                // B
                pitch: 9,
                length: 0,
            },
            Note{
                // Bb
                pitch: 8,
                length: 0,
            },
            Note{
                // Rest
                pitch: 0,
                length: 0,
            },
            Note{
                // Bb
                pitch: 8,
                length: 0,
            },
            Note{
                // Ab
                pitch: 6,
                length: 0,
            },
            Note{
                // Eb
                pitch: 1,
                length: 0,
            },
            Note{
                // F
                pitch: 3,
                length: 0,
            },
            ],
    };

    let bytes: Vec<u8> = song.render().iter().flat_map(|f| f.to_bits().to_be_bytes().to_vec()).collect();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&bytes).unwrap();
}
