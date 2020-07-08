use flate2::read::GzDecoder;
use imagemusic::Song;
use std::env;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        panic!("imagemusic {input song}");
    }
    let song_base64 = &args[0];
    let compressed = base64::decode_config(&song_base64, base64::URL_SAFE_NO_PAD)?;
    let mut decoder = GzDecoder::new(&compressed[..]);
    let mut bincode = Vec::new();
    decoder.read_to_end(&mut bincode)?;
    let song: Song = bincode::deserialize(&bincode)?;
    println!("{:#?}", song);
    /*println!("{:#?}", song);
    let mut output = BufWriter::new(fs::File::create(outputpath)?);
    for sample in song.samples() {
        output.write(&sample.to_be_bytes())?;
    }*/
    Ok(())
}
