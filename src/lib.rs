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

use crate::image::{Image, Payload, Pixel};
use flate2::read::GzDecoder;
use flate2::read::GzEncoder;
use minidom::Element;
use std::io::Read;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;

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
    let song: Song = toml::from_str(toml).map_err(|e| JsValue::from(e.to_string()))?;
    Ok(Box::into_raw(Box::new(song)))
}

/// Read a toml string into a song
#[wasm_bindgen]
pub fn song_from_musicxml(xml: &str) -> Result<*mut Song, JsValue> {
    let root = xml
        .parse::<Element>()
        .map_err(|e| JsValue::from(e.to_string()))?;
    let song = musicxml::from_musicxml(root).map_err(|e| JsValue::from(e.to_string()))?;

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

/// Get all samples from a song
#[wasm_bindgen]
pub fn song_to_toml(song: *mut Song) -> Result<String, JsValue> {
    let song = unsafe { &mut *song };
    // Just allocate enough for a minute
    toml::to_string(&song).map_err(|e| JsValue::from(e.to_string()))
}

/// Read a toml string into a song
#[wasm_bindgen]
pub fn song_from_image(
    image_width: u32,
    image_height: u32,
    image_data: Clamped<Vec<u8>>,
) -> Result<*mut Song, JsValue> {
    let image_data: Vec<Pixel> = image_data
        .chunks_exact(4)
        .map(|chunk| Pixel {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        })
        .collect();

    let image = Image::new((image_width, image_height), image_data);

    let payload = image
        .read_payload()
        .map_err(|e| JsValue::from(e.to_string()))?;
    let payload = payload.data().map_err(|e| JsValue::from(e.to_string()))?;

    let mut decoder = GzDecoder::new(&payload[..]);
    let mut buffer = Vec::new();
    decoder
        .read_to_end(&mut buffer)
        .map_err(|e| JsValue::from(e.to_string()))?;

    let song = bincode::deserialize(&buffer).map_err(|e| JsValue::from(e.to_string()))?;
    Ok(Box::into_raw(Box::new(song)))
}

/// Bake a song into an image.
///
/// Dimensions aren't returned because they are the same as the input ones, so the caller already
/// has them.
#[wasm_bindgen]
pub fn song_bake_image(
    song: *mut Song,
    image_width: u32,
    image_height: u32,
    image_data: Clamped<Vec<u8>>,
) -> Result<Vec<u8>, JsValue> {
    let song = unsafe { &mut *song };

    let bincode = bincode::serialize(&song).map_err(|e| JsValue::from(e.to_string()))?;
    let mut compressed = Vec::new();
    {
        let mut compressor = GzEncoder::new(bincode.as_slice(), flate2::Compression::best());
        compressor
            .read_to_end(&mut compressed)
            .map_err(|e| JsValue::from(e.to_string()))?;
    }

    let payload = Payload::new(&compressed);

    let image_data: Vec<Pixel> = image_data
        .chunks_exact(4)
        .map(|chunk| Pixel {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        })
        .collect();

    let mut image = Image::new((image_width, image_height), image_data);

    image.bake_payload(&payload);

    let image_data: Vec<u8> = image
        .pixels()
        .iter()
        .flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b, pixel.a])
        .collect();

    Ok(image_data)
}
