use imagemusic::musicxml;

use minidom::Element;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 1 {
        panic!("imagemusic {input song}");
    }
    let songpath = &args[0];
    let xml = fs::read_to_string(songpath)?;
    let root: Element = xml.parse()?;
    let _song = musicxml::from_musicxml(root);

    Ok(())
}
