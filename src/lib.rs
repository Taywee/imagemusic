/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

mod envelope;
pub mod error;
mod instrument;
pub mod song;
mod voice;
pub mod parser;

pub use crate::song::Song;
