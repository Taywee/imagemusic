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
    if root.name() != "score-partwise" {
        return Err(Error::InvalidMusicXML("Only partwise musicxml scores are supported")); 
    }

    let first_measure = root
        .children()
        .find(|e| e.name() == "part")
        .ok_or(Error::InvalidMusicXML("Could not find the first part"))?
        .children()
        .find(|e| e.name() == "measure")
        .ok_or(Error::InvalidMusicXML("Could not find the first measure"))?;
    let attributes = first_measure.children()
        .find(|e| e.name() == "attributes")
        .ok_or(Error::InvalidMusicXML("Could not find the attributes"))?;
    let direction = first_measure.children()
        .find(|e| e.name() == "direction")
        .ok_or(Error::InvalidMusicXML("Could not find the direction element"))?;

    // Divisions of quarter notes.
    let divisions: usize = attributes.children()
        .find(|e| e.name() == "divisions")
        .ok_or(Error::InvalidMusicXML("Could not find the divisions element"))?
        .text().parse()?;

    let time = attributes.children()
        .find(|e| e.name() == "time")
        .ok_or(Error::InvalidMusicXML("Could not find the time element"))?;

    // Top of time signature
    let beats: usize = time.children()
        .find(|e| e.name() == "beats")
        .ok_or(Error::InvalidMusicXML("Could not find the beats element"))?
        .text().parse()?;

    // Bottom of time signature
    let beat_type: usize = time.children()
        .find(|e| e.name() == "beat-type")
        .ok_or(Error::InvalidMusicXML("Could not find the beat-type element"))?
        .text().parse()?;

    let tempo: usize = direction.children()
        .find(|e| e.name() == "sound")
        .ok_or(Error::InvalidMusicXML("Could not find the sound element"))?
        .attr("tempo")
        .ok_or(Error::InvalidMusicXML("Could not find the tempo attribute"))?
        .parse()?;

    let divisions_per_measure = divisions * 4 * beats / beat_type;
    let divisions_per_second = (divisions * tempo) as f64 / 60.0;

    unimplemented!()
}
