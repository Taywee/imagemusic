use imagemusic::Song;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        panic!("imagemusic {input song}");
    }
    let song_base64 = &args[0];
    let compressed = base64::decode_config(&song_base64, base64::URL_SAFE_NO_PAD)?;
    let mut bincode: Vec<u8> = Vec::new();
    brotli::BrotliDecompress(&mut &compressed[..], &mut bincode)?;
    let song: Song = bincode::deserialize(&bincode)?;
    println!("{:#?}", song);
    /*println!("{:#?}", song);
    let mut output = BufWriter::new(fs::File::create(outputpath)?);
    for sample in song.samples() {
        output.write(&sample.to_be_bytes())?;
    }*/
    Ok(())
}
