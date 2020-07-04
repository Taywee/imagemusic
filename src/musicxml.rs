mod error;
pub use error::Error;

use crate::note::NoteName;
use crate::Song;
use minidom::Element;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Note {
    duration: usize,
    name: NoteName,
    tie_start: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Measure {
    chords: Vec<Vec<Note>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Part {
    measures: Vec<Measure>,
}

pub fn from_musicxml(root: Element) -> Result<Song, Error> {
    let first_measure = root
        .children()
        .find(|e| e.name() == "part")
        .ok_or(Error::InvalidMusicXML)?
        .children()
        .find(|e| e.name() == "measure")
        .ok_or(Error::InvalidMusicXML)?;
    let attributes = first_measure.children()
        .find(|e| e.name() == "attributes")
        .ok_or(Error::InvalidMusicXML)?;
    let direction = first_measure.children()
        .find(|e| e.name() == "direction")
        .ok_or(Error::InvalidMusicXML)?;

    // Divisions of quarter notes.
    let divisions: usize = attributes.children()
        .find(|e| e.name() == "divisions")
        .ok_or(Error::InvalidMusicXML)?
        .text().parse()?;

    let time = attributes.children()
        .find(|e| e.name() == "time")
        .ok_or(Error::InvalidMusicXML)?;

    // Top of time signature
    let beats: usize = time.children()
        .find(|e| e.name() == "beats")
        .ok_or(Error::InvalidMusicXML)?
        .text().parse()?;

    // Bottom of time signature
    let beat_type: usize = time.children()
        .find(|e| e.name() == "beat-type")
        .ok_or(Error::InvalidMusicXML)?
        .text().parse()?;

    let tempo: usize = direction.children()
        .find(|e| e.name() == "sound")
        .ok_or(Error::InvalidMusicXML)?
        .attr("tempo")
        .ok_or(Error::InvalidMusicXML)?
        .parse()?;

    let divisions_per_measure = divisions * 4 * beats / beat_type;
    let divisions_per_second = (divisions * tempo) as f64 / 60.0;

    unimplemented!()
}
