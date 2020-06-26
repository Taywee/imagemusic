use regex::Regex;
use serde::de;
use serde::ser;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Deref, DerefMut};

/// -is is sharp -es is flat
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NoteName {
    Rest,
    C,
    Cis,
    Cisis,
    Deses,
    Des,
    D,
    Dis,
    Disis,
    Eses,
    Es,
    E,
    Eis,
    Eisis,
    Feses,
    Fes,
    F,
    Fis,
    Fisis,
    Geses,
    Ges,
    G,
    Gis,
    Gisis,
    Ases,
    As,
    A,
    Ais,
    Aisis,
    Beses,
    Bes,
    B,
    Bis,
    Bisis,
    Ceses,
    Ces,
}

impl NoteName {
    pub fn exponent(self) -> i8 {
        use NoteName::*;
        match self {
            Rest => panic!(),
            Ceses => -2,
            Ces => -1,
            C => 0,
            Cis => 1,
            Cisis => 2,
            Deses => 0,
            Des => 1,
            D => 2,
            Dis => 3,
            Disis => 4,
            Eses => 2,
            Es => 3,
            E => 4,
            Eis => 5,
            Eisis => 6,
            Feses => 3,
            Fes => 4,
            F => 5,
            Fis => 6,
            Fisis => 7,
            Geses => 5,
            Ges => 6,
            G => 7,
            Gis => 8,
            Gisis => 9,
            Ases => 7,
            As => 8,
            A => 9,
            Ais => 10,
            Aisis => 11,
            Beses => 9,
            Bes => 10,
            B => 11,
            Bis => 12,
            Bisis => 13,
        }
    }

    /// Construct a note from an input base pitch, from C.
    /// leans sharp.
    /// Panics if pitch is greater than 11.
    fn from_pitch(pitch: u8) -> Self {
        use NoteName::*;
        match pitch {
            0 => C,
            1 => Cis,
            2 => D,
            3 => Dis,
            4 => E,
            5 => F,
            6 => Fis,
            7 => G,
            8 => Gis,
            9 => A,
            10 => Ais,
            11 => B,
            _ => panic!(),
        }
    }

    pub fn name(self) -> &'static str {
        use NoteName::*;
        match self {
            Rest => "r",
            Ceses => "ceses",
            Ces => "ces",
            C => "c",
            Cis => "cis",
            Cisis => "cisis",
            Deses => "deses",
            Des => "des",
            D => "d",
            Dis => "dis",
            Disis => "disis",
            Eses => "eses",
            Es => "es",
            E => "e",
            Eis => "eis",
            Eisis => "eisis",
            Feses => "feses",
            Fes => "fes",
            F => "f",
            Fis => "fis",
            Fisis => "fisis",
            Geses => "geses",
            Ges => "ges",
            G => "g",
            Gis => "gis",
            Gisis => "gisis",
            Ases => "ases",
            As => "as",
            A => "a",
            Ais => "ais",
            Aisis => "aisis",
            Beses => "beses",
            Bes => "bes",
            B => "b",
            Bis => "bis",
            Bisis => "bisis",
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
            "ceses" => Ceses,
            "ces" => Ces,
            "c" => C,
            "cis" => Cis,
            "cisis" => Cisis,
            "deses" => Deses,
            "des" => Des,
            "d" => D,
            "dis" => Dis,
            "disis" => Disis,
            "eses" => Eses,
            "es" => Es,
            "e" => E,
            "eis" => Eis,
            "eisis" => Eisis,
            "feses" => Feses,
            "fes" => Fes,
            "f" => F,
            "fis" => Fis,
            "fisis" => Fisis,
            "geses" => Geses,
            "ges" => Ges,
            "g" => G,
            "gis" => Gis,
            "gisis" => Gisis,
            "ases" => Ases,
            "as" => As,
            "a" => A,
            "ais" => Ais,
            "aisis" => Aisis,
            "beses" => Beses,
            "bes" => Bes,
            "b" => B,
            "bis" => Bis,
            "bisis" => Bisis,
            _ => return Err(NoSuchNoteName),
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Note {
    pub length: u8,
    pub name: NoteName,
    pub octave: u8,
}

impl Note {
    pub fn frequency(self) -> Option<f64> {
        match self.name {
            NoteName::Rest => None,
            name => {
                Some(16.0 * 2.0f64.powf((self.octave as i8 * 12 + name.exponent()) as f64 / 12.0))
            }
        }
    }

    /// Get the pitch as an 8-bit integer.  None is rest, Some(0) is c0, Some(1) is cis0...
    pub fn pitch(self) -> Option<u8> {
        match self.name {
            NoteName::Rest => None,
            name => Some((name.exponent() + self.octave as i8 * 12) as u8),
        }
    }

    pub fn from_length_pitch(length: u8, pitch: Option<u8>) -> Self {
        Note {
            length,
            name: pitch
                .map(|pitch| NoteName::from_pitch(pitch % 12))
                .unwrap_or(NoteName::Rest),
            octave: pitch.unwrap_or(0) / 12,
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.length, self.name.name(), self.octave)
    }
}

impl ser::Serialize for Note {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&self.to_string())
        } else {
            use serde::ser::SerializeTuple;
            let mut tuple = serializer.serialize_tuple(2)?;
            tuple.serialize_element(&self.length)?;
            tuple.serialize_element(&self.pitch().unwrap_or(0))?;
            tuple.end()
        }
    }
}

struct StrNoteVisitor;
struct BinNoteVisitor;

thread_local! {
    static NOTE_PATTERN: Regex = Regex::new(r"^(\d+)(r|([a-g](?:[ie]?s)*)(\d+))$").unwrap();
}

impl<'de> de::Visitor<'de> for StrNoteVisitor {
    type Value = Note;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a {length}{name}{octave} string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let note = NOTE_PATTERN
            .with(|note_pattern| {
                note_pattern.captures(value).map(|captures| {
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
            .ok_or_else(|| de::Error::invalid_value(de::Unexpected::Str(value), &self))?;

        Ok(note)
    }
}

impl<'de> de::Visitor<'de> for BinNoteVisitor {
    type Value = Note;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a ({length}, {pitch}) tuple")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        use de::Error;
        let length: Option<u8> = seq.next_element()?;
        let length = length.ok_or_else(|| A::Error::invalid_length(0, &self))?;
        let pitch: Option<u8> = seq.next_element()?;
        let pitch = pitch.ok_or_else(|| A::Error::invalid_length(1, &self))?;

        let pitch = if pitch == 0 { None } else { Some(pitch) };

        Ok(Note::from_length_pitch(length, pitch))
    }
}

impl<'de> de::Deserialize<'de> for Note {
    fn deserialize<D>(deserializer: D) -> Result<Note, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(StrNoteVisitor)
        } else {
            deserializer.deserialize_tuple(2, BinNoteVisitor)
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
        if serializer.is_human_readable() {
            let notes: Vec<String> = self.iter().map(|note| note.to_string()).collect();
            serializer.serialize_str(&notes.join(" "))
        } else {
            use serde::ser::SerializeSeq;
            let mut seq = serializer.serialize_seq(Some(self.len()))?;
            for note in self.iter() {
                seq.serialize_element(note)?;
            }
            seq.end()
        }
    }
}

struct BinNotesVisitor;
struct StrNotesVisitor;

impl<'de> de::Visitor<'de> for StrNotesVisitor {
    type Value = Notes;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("A series of notes and rests, in {tickcount}{note}{octave} format, or {tickcount}r for rests")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let notes: Result<Vec<Note>, E> = value
            .lines()
            // Filter out comments and blanks
            .filter_map(|line| {
                let line = line.trim();
                match line.chars().next() {
                    None | Some('#') => None,
                    Some(_) => Some(line.split_whitespace()),
                }
            })
            // Join all lines
            .flatten()
            .map(move |s| StrNoteVisitor.visit_str(s))
            .collect();
        notes.map(|notes| Notes(notes))
    }
}

impl<'de> de::Visitor<'de> for BinNotesVisitor {
    type Value = Notes;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("A series of binary notes")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut output = Vec::new();
        loop {
            let note: Option<Note> = seq.next_element()?;
            match note {
                Some(note) => output.push(note),
                None => break,
            }
        }
        Ok(Notes(output))
    }
}

impl<'de> de::Deserialize<'de> for Notes {
    fn deserialize<D>(deserializer: D) -> Result<Notes, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(StrNotesVisitor)
        } else {
            deserializer.deserialize_seq(BinNotesVisitor)
        }
    }
}
