/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

mod instrument;
mod base32;
pub mod error;
mod envelope;
mod voice;
pub mod song;

pub use crate::song::Song;
