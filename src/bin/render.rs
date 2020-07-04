use imagemusic::Song;
use std::env;
use std::fs;
use std::io::{BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        panic!("imagemusic {input song} {output pcm}");
    }
    let (songpath, outputpath) = (&args[0], &args[1]);
    let song_toml = fs::read_to_string(songpath)?;
    let mut song: Song = toml::from_str(&song_toml)?;
    let mut output = BufWriter::new(fs::File::create(outputpath)?);
    for sample in song.samples(44100) {
        output.write(&sample.to_be_bytes())?;
    }
    Ok(())
}
