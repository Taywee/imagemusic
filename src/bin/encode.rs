use asciimusic::Song;
use asciimusic::image::{Payload, Pixel, Image};
use std::env;
use std::fs;
use std::io::Read;
use image::{DynamicImage, RgbaImage};

/// Compress into base64(brotli(bincode(song)))
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() < 3 {
        panic!("asciimusic {input song} {input image} {output image}");
    }
    let songpath = &args[0];
    let inputimagepath = &args[1];
    let outputimagepath = &args[2];
    let song_toml = fs::read_to_string(songpath)?;
    let song: Song = toml::from_str(&song_toml)?;
    let bincode = bincode::serialize(&song)?;
    let mut compressed = Vec::new();
    {
        let mut compressor = brotli::CompressorReader::new(bincode.as_slice(), 4096, 11, 21);
        compressor.read_to_end(&mut compressed)?;
    }

    let payload = Payload::new(&compressed);

    let image = image::open(inputimagepath)?;
    let image = image.into_rgba();
    let dimensions = image.dimensions();

    let pixels: Vec<_> = image.pixels().map(|pixel|
            Pixel {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2],
                a: pixel[3],
            }
        ).collect();

    let mut image = Image::new(dimensions, pixels);
    image.bake_payload(&payload);

    let mut output_image = RgbaImage::new(dimensions.0, dimensions.1);
    //let mut output_image = DynamicImage::new_rgba8(dimensions.0, dimensions.1);

    for (out_pixel, pixel) in output_image.pixels_mut().zip(image.pixels()) {
        out_pixel[0] = pixel.r;
        out_pixel[1] = pixel.g;
        out_pixel[2] = pixel.b;
        out_pixel[3] = pixel.a;
    }

    DynamicImage::ImageRgba8(output_image).save(outputimagepath)?;

    Ok(())
}
