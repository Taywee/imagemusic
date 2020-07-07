#![warn(rust_2018_idioms)]

/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

pub mod envelope;
pub mod image;
pub mod instrument;
pub mod musicxml;
pub mod note;
pub mod song;
pub mod voice;

pub use crate::song::Song;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use crate::image::{Pixel, Image, Payload};
use std::io::Read;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    document.body().expect("document should have a body");

    Ok(())
}

/// Read a toml string into a song
#[wasm_bindgen]
pub fn song_from_toml(toml: &str) -> Result<*mut Song, JsValue> {
    let song: Result<_, JsValue> = toml::from_str(toml).map_err(|e| format!("{}", e).into());
    let song = song?;
    Ok(Box::into_raw(Box::new(song)))
}

#[wasm_bindgen]
pub fn song_free(song: *mut Song) {
    unsafe {
        Box::from_raw(song);
    }
}

/// Get all samples from a song
#[wasm_bindgen]
pub fn song_samples(song: *mut Song, sample_rate: u32) -> Vec<f32> {
    let song = unsafe { &mut *song };
    // Just allocate enough for a minute
    let mut samples = Vec::with_capacity(60 * sample_rate as usize);
    samples.extend(song.samples(sample_rate as usize));
    samples
}

/// Bake a song into an image.
///
/// Dimensions aren't returned because they are the same as the input ones, so the caller already
/// has them.
#[wasm_bindgen]
pub fn song_bake_image(song: *mut Song, image_width: u32, image_height: u32, image_data: Clamped<Vec<u8>>) -> Result<Vec<u8>, JsValue> {
    let song = unsafe { &mut *song };

    let bincode: Result<_, JsValue> = bincode::serialize(&song).map_err(|e| format!("{}", e).into());
    let bincode = bincode?;
    let mut compressed = Vec::new();
    {
        let mut compressor = flate2::read::GzEncoder::new(bincode.as_slice(), flate2::Compression::best());
        let result: Result<_, JsValue> = compressor.read_to_end(&mut compressed).map_err(|e| format!("{}", e).into());
        result?;
    }

    let payload = Payload::new(&compressed);

    let image_data: Vec<Pixel> = image_data.chunks_exact(4).map(|chunk| Pixel {
        r: chunk[0],
        g: chunk[1],
        b: chunk[2],
        a: chunk[3],
    }).collect();

    let mut image = Image::new(
        (image_width, image_height),
        image_data,
    );

    image.bake_payload(&payload);

    let image_data: Vec<u8> = image.pixels().iter().flat_map(|pixel|
        vec![pixel.r, pixel.g, pixel.b, pixel.a]
    ).collect();

    Ok(image_data)
}
