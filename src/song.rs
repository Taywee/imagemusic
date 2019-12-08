use crate::base32::Base32;
use crate::envelope::{Envelope, EnvelopePoint};
use crate::error::LoadError;
use crate::instrument::Instrument;
use crate::voice::{Note, Voice, VoiceIterator};
use std::borrow::Borrow;

/**
 * The entire song as a structure.
 *
 * Contains the base data as well as all the voices.
 */
pub struct Song {
    bps: f64,
    voices: Vec<Voice>,
    sample_rate: f64,
}

pub struct SongIterator<'a> {
    voice_iterators: Vec<VoiceIterator<'a>>,
    volume_modifier: f64,
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

        sample *= self.volume_modifier;

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
    /** Load the song in from an ascii-formatted string.
     */
    pub fn load_from_str(source: &str) -> Result<Song, LoadError> {
        // Spawn a default song
        let mut song = Song {
            bps: 0.0,
            sample_rate: 0.0,
            voices: Vec::new(),
        };

        let mut lines = source
            .split("\n")
            .map(str::trim)
            // Filter empty and comment lines
            .filter(|s| s.len() > 0 && s.chars().next() != Some('#'));

        // Parse the first line for bpm and frequency
        if let Some(first) = lines.next() {
            let parts: Vec<&str> = first.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(LoadError::from("First line must be two numbers"));
            }
            let numbers: Result<Vec<f64>, _> = parts.iter().map(|s| s.parse()).collect();
            match numbers {
                Ok(numbers) => {
                    song.bps = numbers[0];
                    song.sample_rate = numbers[1];
                }
                Err(err) => {
                    return Err(LoadError::from(format!(
                        "Could not convert bps and sample rate: {}",
                        err
                    )));
                }
            }
        } else {
            return Err(LoadError::from("Need first line of bpm and frequency"));
        }

        // Parse following lines for voice data
        for voice_line in lines {
            let parts: Vec<&str> = voice_line.split_whitespace().collect();
            if parts.len() < 5 {
                return Err(LoadError::from("Need 5+ elements per voice: instrument, envelope, volume, base_frequency, and then notes"));
            }
            let instrument_id: u32 = match parts[0].parse() {
                Ok(i) => i,
                Err(e) => {
                    return Err(LoadError::from(format!(
                        "Could not convert instrument id: {}",
                        e
                    )))
                }
            };
            let instrument = match Instrument::from_id(instrument_id) {
                Ok(i) => i,
                Err(()) => {
                    return Err(LoadError::from(format!(
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
                    return Err(LoadError::from(format!(
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
                Err(e) => return Err(LoadError::from(format!("Could not convert volume: {}", e))),
            };

            let start_frequency: f64 = match parts[3].parse() {
                Ok(v) => v,
                Err(e) => {
                    return Err(LoadError::from(format!(
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
                return Err(LoadError::from(
                    "Need an even number of characters for notes",
                ));
            }

            let mut note_iter = notes.chars();

            let mut notes = Vec::new();

            while let Some(pitch_char) = note_iter.next() {
                let pitch = pitch_char.base32_decode()?;
                // We know we have an even number of chars
                let length = note_iter.next().unwrap().base32_decode()?;
                notes.push(Note { pitch, length });
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

    pub fn voice_iterators(&mut self) -> Vec<VoiceIterator> {
        self.voices
            .iter()
            .map(|voice| VoiceIterator {
                instrument: voice.instrument.borrow(),
                envelope: &voice.envelope,
                note_iterator: Box::new(voice.notes.iter()),
                current_note: None,
                note_samples: 0,
                note_current_sample: 0,
                done: false,
                resting: false,
                seconds_per_beat: 1.0 / self.bps,
                sample_rate: self.sample_rate,
                frequency: voice.start_frequency,
                ramp: 0.0,
                volume: voice.volume,
            })
            .collect()
    }

    /** Render the song as f64 samples.
     */
    pub fn samples(&mut self) -> SongIterator {
        let voice_iterators = self.voice_iterators();
        let volume_modifier = 1.0 / (voice_iterators.len() as f64);
        SongIterator {
            voice_iterators,
            volume_modifier,
        }
    }
}
