use crate::envelope::{Envelope, Point};
//use crate::error::{AsciiLoadError, GenerateSamplesError, LoadError};
use crate::instrument::Instrument;
use crate::voice::{Voice, VoiceIterator};
use crate::note::Note;
use std::io::BufReader;
use std::io::Read;
use std::str;
use serde::{Serialize, Deserialize};

/**
 * The entire song as a structure.
 *
 * Contains the base data as well as all the voices.
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Song {
    pub(crate) ticks_per_second: f32,
    #[serde(rename = "voice")]
    pub voices: Vec<Voice>,
}

pub struct SongIterator<'a> {
    voice_iterators: Vec<VoiceIterator<'a>>,
    volume_modifier: f64,
    voice_count: usize,
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

        sample *= self.volume_modifier / self.voice_count as f64;

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
    pub fn voice_iterators(&self) -> Vec<VoiceIterator> {
        self.voices
            .iter()
            .map(|voice| {
                VoiceIterator::new(voice, 1.0 / self.ticks_per_second as f64)
            })
            .collect()
    }

    /** Render the song as f64 samples.
     */
    pub fn samples(&mut self) -> SongIterator {
        let voice_iterators = self.voice_iterators();
        let volume_modifier = 1.0 / (voice_iterators.len() as f64);
        SongIterator {
            voice_count: self.voices.len(),
            voice_iterators,
            volume_modifier,
        }
    }
}
