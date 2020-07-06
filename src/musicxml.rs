/// MusicXML conversion module
mod error;
pub use error::Error;

use crate::note::NoteName;
use minidom::Element;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};

type Voice = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Note {
    duration: usize,
    name: NoteName,
    octave: u8,
    tie_start: bool,
    tie_stop: bool,
    chord: bool,
    voice: Voice,
}

impl From<Note> for crate::note::Note {
    fn from(note: Note) -> Self {
        crate::note::Note {
            length: note.duration as u32,
            name: note.name,
            octave: note.octave,
        }
    }
}

impl Note {
    // Construct a rest of the given duration
    fn rest(duration: usize) -> Self {
        Note {
            duration,
            name: NoteName::Rest,
            chord: false,

            // Voice doesn't matter
            voice: 0,
            octave: 0,
            tie_start: false,
            tie_stop: false,
        }
    }
}

impl TryFrom<&Element> for Note {
    type Error = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let duration = element
            .children()
            .find(|e| e.name() == "duration")
            .ok_or(Error::InvalidMusicXML("Could not find note duration"))?
            .text()
            .parse()?;

        let voice = element
            .children()
            .find(|e| e.name() == "voice")
            .ok_or(Error::InvalidMusicXML("Could not find note voice"))?
            .text()
            .parse()?;

        let chord = matches!(element.children().find(|e| e.name() == "chord"), Some(_));

        let (name, octave) = if let Some(_) = element.children().find(|e| e.name() == "rest") {
            (NoteName::Rest, 0)
        } else {
            let pitch = element
                .children()
                .find(|e| e.name() == "pitch")
                .ok_or(Error::InvalidMusicXML("Could not find note pitch or rest"))?;

            let mut name: NoteName = pitch
                .children()
                .find(|e| e.name() == "step")
                .ok_or(Error::InvalidMusicXML("Could not find note step"))?
                .text()
                .to_lowercase()
                .parse()?;

            let octave = pitch
                .children()
                .find(|e| e.name() == "octave")
                .ok_or(Error::InvalidMusicXML("Could not find note octave"))?
                .text()
                .parse()?;

            let alter: i8 = pitch
                .children()
                .find(|e| e.name() == "alter")
                .map(|alter| alter.text().parse())
                .unwrap_or(Ok(0))?;

            if alter > 0 {
                for _ in 0..alter {
                    name = name.sharpen();
                }
            } else if alter < 0 {
                for _ in alter..0 {
                    name = name.flatten();
                }
            }

            (name, octave)
        };

        let tie_start = matches!(
            element
                .children()
                .find(|e| e.name() == "tie" && e.attr("type") == Some("start")),
            Some(_)
        );
        let tie_stop = matches!(
            element
                .children()
                .find(|e| e.name() == "tie" && e.attr("type") == Some("stop")),
            Some(_)
        );

        Ok(Note {
            duration,
            name,
            tie_start,
            tie_stop,
            voice,
            chord,
            octave,
        })
    }
}

type Chord = Vec<Note>;

// Need the measure so that we can effectively fill in missing voices
#[derive(Debug, Clone, PartialEq, Eq)]
struct Measure {
    notes: HashMap<Voice, Vec<Chord>>,
}

impl TryFrom<&Element> for Measure {
    type Error = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let mut notes = HashMap::new();

        for note in element.children().filter(|e| e.name() == "note") {
            let note: Note = note.try_into()?;
            let voice = notes.entry(note.voice).or_insert(Vec::new());
            if !note.chord {
                // new chord
                voice.push(Vec::new());
            }
            voice
                .last_mut()
                .ok_or(Error::InvalidMusicXML("Chord note on an empty voice"))?
                .push(note);
        }

        Ok(Measure { notes })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    id: Option<String>,
    measures: Vec<Measure>,
}

impl TryFrom<&Element> for Part {
    type Error = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let id = element.attr("id").map(ToString::to_string);
        let measures: Result<_, _> = element
            .children()
            .filter(|e| e.name() == "measure")
            .map(TryInto::try_into)
            .collect();

        Ok(Part {
            id,
            measures: measures?,
        })
    }
}

impl Part {
    pub fn voices(&self) -> HashSet<Voice> {
        self.measures
            .iter()
            .flat_map(|measure| measure.notes.keys())
            .copied()
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Song {
    parts: Vec<Part>,
    measures_completed: bool,
}

impl TryFrom<&Element> for Song {
    type Error = Error;

    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        let parts: Result<_, _> = element
            .children()
            .filter(|e| e.name() == "part")
            .map(TryInto::try_into)
            .collect();

        Ok(Song {
            parts: parts?,
            measures_completed: false,
        })
    }
}

impl Song {
    /// Complete all measures.  That is, no measures will have any missing voices.  Any missing
    /// voices in a measure will be filled with a measure-long rest.  Because the song doesn't know
    /// measure duration innately, that value must be passed in.
    fn complete_measures(&mut self, measure_duration: usize) {
        self.measures_completed = true;
        let measure_rest = Note::rest(measure_duration);

        for part in &mut self.parts {
            let voices = part.voices();
            for measure in &mut part.measures {
                for voice in &voices {
                    measure
                        .notes
                        .entry(*voice)
                        .or_insert_with(|| vec![vec![measure_rest]]);
                }
            }
        }
    }

    /// Flatten a song, removing parts, voices, and measures, getting the entire song as a vector
    /// of voices, each of which is a vector of chords.
    ///
    /// Expects all parts and measures to be complete.  If complete_measures hasn't been called,
    /// this will panic.
    fn as_voices(&self) -> Vec<Vec<Chord>> {
        if !self.measures_completed {
            panic!("Measures must be completed!");
        }
        let mut output = Vec::new();
        for part in &self.parts {
            let mut voice_notes: HashMap<Voice, Vec<Chord>> = HashMap::new();
            for measure in &part.measures {
                for (voice, notes) in &measure.notes {
                    voice_notes
                        .entry(*voice)
                        .or_insert_with(|| Vec::new())
                        .extend_from_slice(notes);
                }
            }
            for (_, notes) in voice_notes.into_iter() {
                output.push(notes);
            }
        }
        output
    }
}

pub fn from_musicxml(root: Element) -> Result<crate::Song, Error> {
    if root.name() != "score-partwise" {
        return Err(Error::InvalidMusicXML(
            "Only partwise musicxml scores are supported",
        ));
    }

    let first_measure = root
        .children()
        .find(|e| e.name() == "part")
        .ok_or(Error::InvalidMusicXML("Could not find the first part"))?
        .children()
        .find(|e| e.name() == "measure")
        .ok_or(Error::InvalidMusicXML("Could not find the first measure"))?;
    let attributes = first_measure
        .children()
        .find(|e| e.name() == "attributes")
        .ok_or(Error::InvalidMusicXML("Could not find the attributes"))?;
    let direction = first_measure
        .children()
        .find(|e| e.name() == "direction")
        .ok_or(Error::InvalidMusicXML(
            "Could not find the direction element",
        ))?;

    // Divisions of quarter notes.
    let divisions: usize = attributes
        .children()
        .find(|e| e.name() == "divisions")
        .ok_or(Error::InvalidMusicXML(
            "Could not find the divisions element",
        ))?
        .text()
        .parse()?;

    let time = attributes
        .children()
        .find(|e| e.name() == "time")
        .ok_or(Error::InvalidMusicXML("Could not find the time element"))?;

    // Top of time signature
    let beats: usize = time
        .children()
        .find(|e| e.name() == "beats")
        .ok_or(Error::InvalidMusicXML("Could not find the beats element"))?
        .text()
        .parse()?;

    // Bottom of time signature
    let beat_type: usize = time
        .children()
        .find(|e| e.name() == "beat-type")
        .ok_or(Error::InvalidMusicXML(
            "Could not find the beat-type element",
        ))?
        .text()
        .parse()?;

    let tempo: usize = direction
        .children()
        .find(|e| e.name() == "sound")
        .ok_or(Error::InvalidMusicXML("Could not find the sound element"))?
        .attr("tempo")
        .ok_or(Error::InvalidMusicXML("Could not find the tempo attribute"))?
        .parse()?;

    let divisions_per_measure = divisions * 4 * beats / beat_type;
    let divisions_per_second = (divisions * tempo) as f32 / 60.0;

    // id -> name
    let mut part_names: HashMap<String, String> = HashMap::new();

    for part_list in root.children().find(|e| e.name() == "part-list") {
        for score_part in part_list.children().filter(|e| e.name() == "score-part") {
            for id in score_part.attr("id") {
                for name in score_part.children().find(|e| e.name() == "part-name") {
                    part_names.insert(String::from(id), name.text());
                }
            }
        }
    }

    let mut song: Song = (&root).try_into()?;

    song.complete_measures(divisions_per_measure);

    // Turn a Song into a vec of voices, dropping measures and parts
    let voices = song.as_voices();


    // Totally disassembled voices, with all chords torn apart.
    let mut output_voices = Vec::new();

    for voice in voices {
        let mut note_offset = 0;
        // chords in voice split into individual tracks
        let mut chord_voices: Vec<Vec<Note>> = Vec::new();
        for chord in voice {
            // voices to be added after awaiting_voices is dropped
            let mut new_voices = Vec::new();

            // All chord voices are expected to be changed every single chord.  Either they are a
            // tied note awaiting an end or an untied that is awaiting a new note.  If neither
            // condition is met, an existing tie is extended and an existing note as appended with
            // an implicit rest.
            let mut awaiting_voices: Vec<&mut Vec<Note>> = chord_voices.iter_mut().collect();
            for note in &chord {
                if note.tie_stop {
                    match awaiting_voices.iter_mut().enumerate().find(|(_, track)| {
                        let track_note = track.last().unwrap();
                        track_note.tie_start
                            && track_note.name == note.name
                            && track_note.octave == note.octave
                    }) {
                        Some((index, track)) => {
                            track.last_mut().unwrap().duration += note.duration;
                            // If it is a stop+start, the note continues, otherwise it ends
                            if !note.tie_start {
                                track.last_mut().unwrap().tie_start = false;
                            }

                            // This voice is not awaiting a new note for this chord anymore
                            awaiting_voices.remove(index);
                        }
                        None => {
                            return Err(Error::InvalidMusicXML(
                                "tie stop has no tie start to continue from",
                            ))
                        }
                    }
                } else {
                    match awaiting_voices
                        .iter_mut()
                        .enumerate()
                        .find(|(_, track)| !track.last().unwrap().tie_start)
                    {
                        Some((index, track)) => {
                            track.push(*note);
                            // This voice is not awaiting a new note for this chord anymore
                            awaiting_voices.remove(index);
                        }
                        None => {
                            let mut new_voice = Vec::new();
                            // If this chord begins a new voice, it may need to begin with a long
                            // rest
                            if note_offset > 0 {
                                new_voice.push(Note::rest(note_offset));
                            }

                            new_voice.push(*note);
                            new_voices.push(new_voice);
                        }
                    }
                }
            }
            let duration = chord[0].duration;
            // Remaining awaiting voices either get their ties extended or a rest inserted
            for voice in awaiting_voices {
                if voice.last().unwrap().tie_start {
                    voice.last_mut().unwrap().duration += duration;
                } else {
                    voice.push(Note::rest(duration));
                }
            }
            note_offset += duration;

            chord_voices.extend_from_slice(&new_voices);
        }

        output_voices.extend_from_slice(&chord_voices);
    }

    // TODO: exit error if there are any uncompleted ties
    // TODO: optimize rests by combining adjacent ones in any voice

    Ok(crate::Song {
        ticks_per_second: divisions_per_second,
        voices: output_voices.into_iter().map(|notes| crate::voice::Voice {
            volume: u8::MAX,
            instrument: crate::instrument::Instrument::Sawtooth,
            notes: crate::note::Notes(notes.into_iter().map(Note::into).collect()),
            envelope: Default::default(),
        }).collect(),
    })
}
