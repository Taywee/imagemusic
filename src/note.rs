use regex::Regex;
use serde::de;
use serde::ser;
use std::fmt;
use std::ops::{Deref, DerefMut};

/// -is is sharp -es is flat
#[derive(Debug, Clone, Copy)]
pub enum NoteName {
    Rest,
    C,
    Cis,
    Des,
    D,
    Dis,
    Es,
    E,
    Eis,
    Fes,
    F,
    Fis,
    Ges,
    G,
    Gis,
    As,
    A,
    Ais,
    Bes,
    B,
    Bis,
    Ces,
}

impl NoteName {
    pub fn exponent(self) -> i8 {
        use NoteName::*;
        match self {
            Rest => panic!(),
            Ces => -1,
            C => 0,
            Cis => 1,
            Des => 1,
            D => 2,
            Dis => 3,
            Es => 3,
            E => 4,
            Eis => 5,
            Fes => 4,
            F => 5,
            Fis => 6,
            Ges => 6,
            G => 7,
            Gis => 8,
            As => 8,
            A => 9,
            Ais => 10,
            Bes => 10,
            B => 11,
            Bis => 12,
        }
    }

    pub fn name(self) -> &'static str {
        use NoteName::*;
        match self {
            Rest => "r",
            Ces => "ces",
            C => "c",
            Cis => "cis",
            Des => "des",
            D => "d",
            Dis => "dis",
            Es => "es",
            E => "e",
            Eis => "eis",
            Fes => "fes",
            F => "f",
            Fis => "fis",
            Ges => "ges",
            G => "g",
            Gis => "gis",
            As => "as",
            A => "a",
            Ais => "ais",
            Bes => "bes",
            B => "b",
            Bis => "bis",
        }
    }
}

#[derive(Debug)]
pub struct NoSuchNoteName;

impl std::str::FromStr for NoteName {
    type Err = NoSuchNoteName;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NoteName::*;
        Ok(match s {
            "r" => Rest,
            "ces" => Ces,
            "c" => C,
            "cis" => Cis,
            "des" => Des,
            "d" => D,
            "dis" => Dis,
            "es" => Es,
            "e" => E,
            "eis" => Eis,
            "fes" => Fes,
            "f" => F,
            "fis" => Fis,
            "ges" => Ges,
            "g" => G,
            "gis" => Gis,
            "as" => As,
            "a" => A,
            "ais" => Ais,
            "bes" => Bes,
            "b" => B,
            "bis" => Bis,
            _ => return Err(NoSuchNoteName),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Note {
    pub length: u8,
    pub name: NoteName,
    pub octave: i8,
}

impl Note {
    pub fn frequency(self) -> Option<f64> {
        match self.name {
            NoteName::Rest => None,
            name => Some(16.0 * 2.0f64.powf((self.octave * 12 + name.exponent()) as f64 / 12.0)),
        }
    }

    /// Get the pitch as an 8-bit integer.  0 is rest, 1 is c0, 1 is cis0...
    pub fn pitch(self) -> u8 {
        match self.name {
            NoteName::Rest => 0,
            // try an option or non-option version
            name => (name.exponent() + self.octave as i8 * 12) as u8 + 1
        }
    }
}

/// Simple wrapper around a note sequence that allows for compact string-only serialization and
/// deserialization
#[derive(Debug, Clone)]
pub struct Notes(pub Vec<Note>);

impl Deref for Notes {
    type Target = [Note];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl DerefMut for Notes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.deref_mut()
    }
}

impl ser::Serialize for Notes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use serde::ser::{SerializeSeq, SerializeTuple};
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for note in self.iter() {
            //let mut tup = serializer.serialize_tuple(2)?;
            seq.serialize_element(&(((note.length as u8) << 4) | note.pitch()))?;
            //tup.end()?;
        }
        seq.end()
    }
}

struct NotesVisitor;

// TODO: use is_human_readable to switch between compact and non-compact representation
impl<'de> de::Visitor<'de> for NotesVisitor {
    type Value = Notes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A series of notes and rests, in {tickcount}{note}{octave} format, or {tickcount}r for rests")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let note_pattern = Regex::new(r"^(\d+)(r|([a-g](?:[ie]?s)*)(\d+))$").unwrap();

        let notes: Vec<Note> = value
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                match line.chars().next() {
                    None | Some('#') => None,
                    Some(_) => Some(line.split_whitespace()),
                }
            })
            .flatten()
            .filter_map(move |s| {
                note_pattern.captures(s).map(|captures| {
                    let length: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let (name, octave) = match captures.get(2).unwrap().as_str() {
                        "r" => (NoteName::Rest, 0),
                        _ => (
                            captures.get(3).unwrap().as_str().parse().unwrap(),
                            captures.get(4).unwrap().as_str().parse().unwrap(),
                        ),
                    };
                    Note {
                        length,
                        name,
                        octave,
                    }
                })
            })
            .collect();

        Ok(Notes(notes))
    }
}

impl<'de> de::Deserialize<'de> for Notes {
    fn deserialize<D>(deserializer: D) -> Result<Notes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(NotesVisitor)
    }
}
