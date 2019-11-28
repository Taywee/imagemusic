extern crate asciimusic;

use asciimusic::{Song, Note, Voice, Sawtooth, Sine, Square, Triangle, Envelope, EnvelopePoint};
use std::io::prelude::*;
use std::io;

fn main() {
    let mut song = Song {
        // quarter notes per minute -> quarter notes per second -> eighth notes per second
        bps: 180.0 / 60.0 * 2.0,
        sample_rate: 48_000.0,
        voices: vec!{
            Voice {
                envelope: Envelope{
                    points: vec!{
                        EnvelopePoint{
                            amplitude: 0.0,
                            stop: 0.0,
                        },
                        EnvelopePoint{
                            amplitude: 1.0,
                            stop: 0.001,
                        },
                        EnvelopePoint{
                            amplitude: 1.0,
                            stop: -0.01,
                        },
                        EnvelopePoint{
                            amplitude: 0.0,
                            stop: -0.001,
                        }
                    },
                },
                volume: 1.0,
                // Scientific pitch C5
                start_frequency: 512.0,
                instrument: Box::new(Sine{}),
                notes: vec!{
                    Note{
                        // E
                        pitch: 4,
                        length: 0,
                    },
                    Note{
                        // E
                        pitch: 0,
                        length: 0,
                    },
                    Note{
                        pitch: -16,
                        length: 0,
                    },
                    Note{
                        // E
                        pitch: 0,
                        length: 0,
                    },
                    Note{
                        pitch: -16,
                        length: 0,
                    },
                    Note{
                        // C
                        pitch: -4,
                        length: 0,
                    },
                    Note{
                        // E
                        pitch: 4,
                        length: 1,
                    },
                    Note{
                        // G
                        pitch: 3,
                        length: 1,
                    },
                    Note{
                        pitch: -16,
                        length: 1,
                    },
                    Note{
                        // G
                        pitch: -12,
                        length: 1,
                    },
                    Note{
                        pitch: -16,
                        length: 1,
                    },
                },
            },
        }
    };

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for sample in song.iter() {
        handle.write_all(&dbg!(sample).to_bits().to_be_bytes()).unwrap();
    }
}
