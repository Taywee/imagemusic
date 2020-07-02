#![warn(rust_2018_idioms)]

/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

pub mod envelope;
pub mod image;
pub mod instrument;
pub mod note;
pub mod song;
pub mod voice;

pub use crate::song::Song;

use song::SongIterator;
use wasm_bindgen::prelude::*;
use std::ffi::c_void;

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

#[wasm_bindgen]
pub fn get_bloody_tears() -> Result<*mut Song, JsValue> {
    let bloody_tears_toml = include_str!("../bloodytears.toml");
    let song: Result<_, JsValue> =
        toml::from_str(bloody_tears_toml).map_err(|e| format!("{}", e).into());
    let song = song?;
    Ok(Box::into_raw(Box::new(song)))
}

#[wasm_bindgen]
pub fn song_free(song: *mut Song) {
    unsafe {
        Box::from_raw(song);
    }
}

#[wasm_bindgen]
pub fn song_samples(song: *mut Song) -> *mut c_void {
    let song = unsafe {
        &mut *song
    };
    let samples = Box::into_raw(Box::new(song.samples()));
    samples as *mut c_void
}

#[wasm_bindgen]
pub fn samples_free(samples: *mut c_void) {
    unsafe {
        Box::from_raw(samples as *mut SongIterator<'_>);
    }
}

#[wasm_bindgen]
pub fn samples_next(samples: *mut c_void) -> JsValue {
    let samples = unsafe {
        &mut *(samples as *mut SongIterator<'_>)
    };
    match samples.next() {
        Some(sample) => JsValue::from_f64(sample),
        None => JsValue::null(),
    }
}
