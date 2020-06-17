extern crate asciimusic;

use asciimusic::Song;
use std::io::BufWriter;
use std::io::prelude::*;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 2 {
        panic!("asciimusic {input song} {output pcm}");
    }
    let (songpath, outputpath) = (&args[0], &args[1]);
    let song_toml = fs::read_to_string(songpath)?;
    let mut song: Song = toml::from_str(&song_toml)?;
    let mut bincode = bincode::serialize(&song)?;
    println!("Song bincode: {}", bincode.len());
    let mut song: Song = bincode::deserialize(&bincode)?;
    println!("Song: {:#?}", song);
    let mut compressor = brotli::CompressorReader::new(bincode.as_slice(), 4096, 11, 21);
    let mut compressed = Vec::new();
    compressor.read_to_end(&mut compressed)?;
    println!("compressed: {}", compressed.len());
    let encoded = base64::encode_config(compressed, base64::URL_SAFE_NO_PAD);
    println!("encoded: {}", encoded.len());
    println!("{}", encoded);
    /*println!("{:#?}", song);
    let mut output = BufWriter::new(fs::File::create(outputpath)?);
    for sample in song.samples() {
        output.write(&sample.to_be_bytes())?;
    }*/
    Ok(())
}
