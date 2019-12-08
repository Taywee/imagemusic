/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

mod base32;
mod envelope;
pub mod error;
mod instrument;
pub mod song;
mod voice;

pub use crate::song::Song;
