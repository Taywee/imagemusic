use crate::envelope::{Envelope, EnvelopePoint};
use crate::error::{AsciiLoadError, GenerateSamplesError, LoadError, MusicXMLLoadError};
use crate::instrument::Instrument;
use crate::voice::{Note, Voice, VoiceIterator};
use std::io::BufReader;
use std::io::Read;
use std::str;

/**
 * The entire song as a structure.
 *
 * Contains the base data as well as all the voices.
 */
pub struct Song {
    ticks_per_second: f32,
    voices: Vec<Voice>,
    sample_rate: u64,
}

pub struct SongIterator<'a> {
    voice_iterators: Vec<VoiceIterator<'a>>,
    volume_modifier: f64,
    voice_chord_sum: usize,
}

impl<'a> Iterator for SongIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.voice_iterators.is_empty() {
            return None;
        }

        let mut sample = 0.0;
        let mut removals = Vec::with_capacity(self.voice_iterators.len());
        for (i, voice_iterator) in self.voice_iterators.iter_mut().enumerate() {
            if let Some(voice) = voice_iterator.next() {
                sample += voice;
            } else {
                removals.push(i);
            }
        }

        for removal in removals.into_iter().rev() {
            self.voice_iterators.remove(removal);
        }

        sample *= self.volume_modifier / self.voice_chord_sum as f64;

        if sample > 1.0 {
            Some(1.0)
        } else if sample < -1.0 {
            Some(-1.0)
        } else {
            Some(sample)
        }
    }
}

impl Song {
    /**
     * Load the song in from an ascii-formatted source.
     */
    pub fn load_ascii(source: &mut dyn Read) -> Result<Song, AsciiLoadError> {
        // Spawn a default song
        let mut song = Song {
            ticks_per_second: 0.0,
            sample_rate: 0,
            voices: Vec::new(),
        };

        let lines: Vec<String> = {
            let mut vec = Vec::new();
            source.read_to_end(&mut vec)?;
            str::from_utf8(&vec)?
                .split("\n")
                .map(str::trim)
                // Filter empty and comment lines
                .filter(|s| s.len() > 0 && s.chars().next() != Some('#'))
                .map(String::from)
                .collect()
        };
        let mut lines = lines.into_iter();

        // Parse the first line for bpm and frequency
        if let Some(first) = lines.next() {
            let parts: Vec<&str> = first.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(AsciiLoadError::from("First line must be two numbers"));
            }
            let numbers: Result<Vec<f64>, _> = parts.iter().map(|s| s.parse()).collect();
            match numbers {
                Ok(numbers) => {
                    song.ticks_per_second = numbers[0];
                    song.sample_rate = numbers[1];
                }
                Err(err) => {
                    return Err(AsciiLoadError::from(format!(
                        "Could not convert bps and sample rate: {}",
                        err
                    )));
                }
            }
        } else {
            return Err(AsciiLoadError::from("Need first line of bpm and frequency"));
        }

        // Parse following lines for voice data
        for voice_line in lines {
            let parts: Vec<&str> = voice_line.split_whitespace().collect();
            if parts.len() < 5 {
                return Err(AsciiLoadError::from("Need 5+ elements per voice: instrument, envelope, volume, base_frequency, and then notes"));
            }
            let instrument_id: u32 = match parts[0].parse() {
                Ok(i) => i,
                Err(e) => {
                    return Err(AsciiLoadError::from(format!(
                        "Could not convert instrument id: {}",
                        e
                    )))
                }
            };
            let instrument = match Instrument::from_id(instrument_id) {
                Ok(i) => i,
                Err(()) => {
                    return Err(AsciiLoadError::from(format!(
                        "Instrument id {} not recognized",
                        instrument_id
                    )))
                }
            };
            let envelope: Result<Vec<Vec<f64>>, _> = parts[1]
                .split("/")
                .map(|s| s.split(",").map(|s| s.parse()).collect())
                .collect();

            let envelope_str = match envelope {
                Ok(e) => e,
                Err(e) => {
                    return Err(AsciiLoadError::from(format!(
                        "Could not convert envelope: {}",
                        e
                    )))
                }
            };

            let envelope_points: Result<Vec<EnvelopePoint>, String> = envelope_str
                .iter()
                .map(|points| {
                    if points.len() == 2 {
                        Ok(EnvelopePoint {
                            amplitude: points[0],
                            stop: points[1],
                        })
                    } else {
                        Err(format!(
                            "Needed two points, but got {} for {:?}",
                            points.len(),
                            points
                        ))
                    }
                })
                .collect();

            let envelope = Envelope {
                points: envelope_points?,
            };

            let volume: f64 = match parts[2].parse() {
                Ok(v) => v,
                Err(e) => {
                    return Err(AsciiLoadError::from(format!(
                        "Could not convert volume: {}",
                        e
                    )))
                }
            };

            let start_frequency: f64 = match parts[3].parse() {
                Ok(v) => v,
                Err(e) => {
                    return Err(AsciiLoadError::from(format!(
                        "Could not convert frequency: {}",
                        e
                    )))
                }
            };

            let notes: String = parts
                .iter()
                .skip(4)
                .flat_map(|block| block.chars())
                .collect();

            if notes.len() % 2 != 0 {
                return Err(AsciiLoadError::from(
                    "Need an even number of characters for notes",
                ));
            }

            let mut note_iter = notes.chars();

            let mut notes = Vec::new();

            while let Some(pitch_char) = note_iter.next() {
                let pitch = pitch_char.base32_decode()?;
                // We know we have an even number of chars
                let length = note_iter.next().unwrap().base32_decode()?;
                notes.push(Note {
                    pitches: vec![pitch],
                    length,
                });
            }

            song.voices.push(Voice {
                envelope,
                instrument,
                notes,
                start_frequency,
                volume,
            });
        }

        Ok(song)
    }

    /**
     * Load the song in from a musicxml-formatted source (not compressed)
     *
     * Will have to load in interestingly.  Need to have a map of voices, mapping from a tuple or
     * struct of part id, voice id, and position in chord.
     * Need to figure out how missing measures and beats are considered by the standard.  Are
     * voices interleaved?  Will we have to fill in rests?
     * As we go, we will add voices and beats.  Need to figure out how to handle chords.  It will
     * have to be handled as a separate voice regardless, as the format doesn't allow chords to be
     * specified.
     *
     * Options:
     *
     *  * Allow a single voice to handle chords, which can be stacked.  Volume might make this
     *  difficult, as we then don't know how much to drop the volume.
     *  
     *      * Can't post-process volume, because the API relies on the ability to stream out samples.
     *      * Maybe just make volume configurable, but that could easily be done by the consuming
     *      API.
     *      * Probably have each voice maintain a value for its max chord size.
     *      * This also will have issues because the canonical ascii format will have to be able to
     *      encode stacked notes.  This would make things a little easier with the musicxml
     *      interop, though.
     *
     *  * Track current position per part and voice, because every note needs a part and a voice,
     * but not necessarily a chord.  All chords are coordinated by different voices.
     *
     *      * Each non-chord note advances the position by its length.  Each chord note does not
     *      advance the position.  When a non-existent voice would come in, check the current
     *      position and back-fill with rests.
     *
     *  * A mix of the two.  A single voice handles chords, but this is not a feature in the ascii
     *  format, which just encodes it all as different voices, and loads all voices with
     *  single-note runs.
     */
    pub fn load_uncompressed_musicxml(source: &mut dyn Read) -> Result<Song, MusicXMLLoadError> {
        // Spawn a default song
        let song = Song {
            ticks_per_second: 0.0,
            sample_rate: 0.0,
            voices: Vec::new(),
        };

        let buf_reader = BufReader::new(source);
        let mut reader = Reader::from_reader(buf_reader);
        let dom = Element::from_reader(&mut reader)?;

        if dom.name() != "score-partwise" {
            return Err(MusicXMLLoadError::from(format!(
                "Could not load musicxml with root element {}.  Need score-partwise.",
                dom.name()
            )));
        }

        Ok(song)
    }

    pub fn voice_iterators(&self) -> Result<Vec<VoiceIterator>, LoadError> {
        self.voices
            .iter()
            .map(|voice| {
                VoiceIterator::new(voice, 1.0 / self.ticks_per_second, self.sample_rate).map_err(LoadError::from)
            })
            .collect()
    }

    /** Render the song as f64 samples.
     */
    pub fn samples(&mut self) -> Result<SongIterator, LoadError> {
        let voice_iterators = self.voice_iterators()?;
        let volume_modifier = 1.0 / (voice_iterators.len() as f64);
        let voice_chord_sum = self
            .voices
            .iter()
            .map(|v| v.largest_chord().ok_or(GenerateSamplesError::EmptyVoice))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|n| n.len())
            .sum();
        Ok(SongIterator {
            voice_chord_sum,
            voice_iterators,
            volume_modifier,
        })
    }
}
