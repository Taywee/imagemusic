#![warn(rust_2018_idioms)]

/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

pub mod envelope;
pub mod instrument;
pub mod note;
pub mod song;
pub mod voice;

pub use crate::song::Song;
