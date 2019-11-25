extern crate asciimusic;

use asciimusic::{Song, Note, Voice, Sawtooth, Sine, Square, Triangle};
use std::io::prelude::*;
use std::io;

fn main() {
    let mut song = Song {
        // quarter notes per minute -> quarter notes per second -> sixteeth notes per second
        bps: 115.0 / 60.0 * 4.0,
        sample_rate: 48_000.0,
        voices: vec!{
            Voice {
                volume: 1.0,
                // Scientific pitch C5
                start_frequency: 512.0,
                instrument: Box::new(Triangle{}),
                notes: vec!{
                    Note{
                        pitch: 5,
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
                },
            },
            Voice {
                volume: 1.0,
                // Scientific pitch C5
                start_frequency: 512.0,
                instrument: Box::new(Sine{}),
                notes: vec!{
                    Note{
                        // C
                        pitch: 0,
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
                        // C
                        pitch: -5,
                        length: 1,
                    },
                    Note{
                        // Rest
                        pitch: -16,
                        length: 0,
                    },
                    Note{
                        // Ab
                        pitch: 8,
                        length: 0,
                    },
                    Note{
                        // F
                        pitch: -3,
                        length: 0,
                    },
                    Note{
                        // Rest
                        pitch: -16,
                        length: 0,
                    },
                    Note{
                        // G
                        pitch: 2,
                        length: 0,
                    },
                    Note{
                        // Fb
                        pitch: -3,
                        length: 0,
                    },
                    Note{
                        // Bb
                        pitch: -6,
                        length: 0,
                    },
                    Note{
                        // Db
                        pitch: 3,
                        length: 0,
                    },
                },
            },
            Voice {
                volume: 0.25,
                start_frequency: 64.0,
                instrument: Box::new(Sawtooth{}),
                notes: vec!{
                    Note{
                        pitch: 5,
                        length: 0,
                    },
                    Note{
                        pitch: 7,
                        length: 0,
                    },
                    Note{
                        pitch: -2,
                        length: 0,
                    },
                    Note{
                        pitch: 2,
                        length: 0,
                    },
                    Note{
                        pitch: -7,
                        length: 0,
                    },
                    Note{
                        pitch: 7,
                        length: 0,
                    },
                    Note{
                        pitch: -2,
                        length: 0,
                    },
                    Note{
                        pitch: 2,
                        length: 0,
                    },
                    Note{
                        pitch: -11,
                        length: 0,
                    },
                    Note{
                        pitch: 10,
                        length: 0,
                    },
                    Note{
                        pitch: -1,
                        length: 0,
                    },
                    Note{
                        pitch: 1,
                        length: 0,
                    },
                    Note{
                        pitch: -11,
                        length: 0,
                    },
                    Note{
                        pitch: 12,
                        length: 0,
                    },
                    Note{
                        pitch: -5,
                        length: 0,
                    },
                    Note{
                        pitch: 3,
                        length: 0,
                    },
                },
            },
        }
    };

    let bytes: Vec<u8> = song.render().iter().flat_map(|f| f.to_bits().to_be_bytes().to_vec()).collect();

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(&bytes).unwrap();
}
