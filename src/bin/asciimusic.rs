extern crate asciimusic;

use asciimusic::Song;
use asciimusic::error::LoadError;
use std::io::prelude::*;
use std::io;

fn main() -> Result<(), LoadError> {
    let input = r#"
    6 48000
    # Sawtooth, Square, Sine, Triangle
    1 0,0/1,0.005/0.75,0.01/0,-0.01 1 512 UA QA AA QA AA MA UB TB AB EB AB
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
