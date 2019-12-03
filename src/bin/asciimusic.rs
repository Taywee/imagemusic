extern crate asciimusic;

use asciimusic::{Song, Note, Voice, Sawtooth, Sine, Square, Triangle, Envelope, EnvelopePoint};
use std::io::prelude::*;
use std::io;

fn main() -> Result<(), String> {
    /*let mut song = Song {
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
                            stop: 0.005,
                        },
                        EnvelopePoint{
                            amplitude: 0.75,
                            stop: 0.01,
                        },
                        EnvelopePoint{
                            amplitude: 0.75,
                            stop: -0.02,
                        },
                        EnvelopePoint{
                            amplitude: 0.0,
                            stop: -0.01,
                        }
                    },
                },
                volume: 1.0,
                // Scientific pitch C5
                start_frequency: 512.0,
                instrument: Box::new(Square),
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
    };*/

    let input = r#"
    6 48000
    # Sawtooth, Square, Sine, Triangle
    1 0,0/1,0.005/0.75,0.01/0.75,-0.02/0,-0.01 1 512 UQQQAQQQAQMQURTRARERAR

    #1 0,0 1,0.005 0.75,0.01 0.75,-0.02 0,-0.01
    #1 512 UQQQAQQQAQMQURTRARERAR
    "#;

    let mut song = Song::load_from_str(&input)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for sample in song.iter() {
        handle.write_all(&sample.to_bits().to_be_bytes()).unwrap();
    }
    Ok(())
}

/* table:
-16=A       -8=I       0=Q        8=Y
-15=B       -7=J       1=R        9=Z
-14=C       -6=K       2=S       10=2
-13=D       -5=L       3=T       11=3
-12=E       -4=M       4=U       12=4
-11=F       -3=N       5=V       13=5
-10=G       -2=O       6=W       14=6
 -9=H       -1=P       7=X       15=7 
*/
