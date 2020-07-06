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

// Get all samples from a song
#[wasm_bindgen]
pub fn song_samples(song: *mut Song, sample_rate: u32) -> Vec<f32> {
    let song = unsafe { &mut *song };
    log("getting samples");
    let samples = song.samples(sample_rate as usize).collect();
    log("got samples");
    samples
}
