use asciimusic::Song;
use std::env;
use std::fs;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        panic!("asciimusic {input song}");
    }
    let songpath = &args[0];
    let song_toml = fs::read_to_string(songpath)?;
    let song: Song = toml::from_str(&song_toml)?;
    let bincode = bincode::serialize(&song)?;
    let mut compressor = brotli::CompressorReader::new(bincode.as_slice(), 4096, 11, 21);
    let mut compressed = Vec::new();
    compressor.read_to_end(&mut compressed)?;
    let encoded = base64::encode_config(compressed, base64::URL_SAFE_NO_PAD);
    println!("{}", encoded);
    /*println!("{:#?}", song);
    let mut output = BufWriter::new(fs::File::create(outputpath)?);
    for sample in song.samples() {
        output.write(&sample.to_be_bytes())?;
    }*/
    Ok(())
}
