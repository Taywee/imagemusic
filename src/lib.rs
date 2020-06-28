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

#[wasm_bindgen]
pub fn parse_song(input: String) -> Result<*mut Song, JsValue> {
    let compressed: Result<_, JsValue> =
        base64::decode_config(input, base64::URL_SAFE_NO_PAD).map_err(|e| format!("{}", e).into());
    let mut bincode: Vec<u8> = Vec::new();
    let result: Result<_, JsValue> = brotli::BrotliDecompress(&mut &compressed?[..], &mut bincode)
        .map_err(|e| format!("{}", e).into());
    result?;
    let song: Result<_, JsValue> =
        bincode::deserialize(&bincode).map_err(|e| format!("{}", e).into());
    let song = song?;
    log(&format!("{:#?}", song));
    Ok(Box::into_raw(Box::new(song)))
}
