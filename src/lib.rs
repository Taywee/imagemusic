/*!
 * The main entry point to this crate is [`song::Song`](song/struct.Song.html)
 */

mod envelope;
mod instrument;
mod note;
pub mod song;
mod voice;

pub use crate::song::Song;
