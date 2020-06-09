use crate::envelope::Envelope;
use crate::instrument::Instrument;
use crate::note::{Note, Notes};
use serde::{Deserialize, Serialize};

const SAMPLE_RATE: f64 = 44100.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    pub volume: u8,
    //#[serde(default = "Instrument::Sine")]
    pub instrument: Instrument,
    pub notes: Notes,
    pub envelope: Envelope,
}

/**
 * Iterate through all the samples in a voice.
 *
 * This does not return a bound value.
 * Its output should be scaled based on the voice's chord size.
 */
pub struct VoiceIterator<'a> {
    pub instrument: Instrument,
    pub envelope: &'a Envelope,
    pub note_iterator: Box<dyn Iterator<Item = &'a Note> + 'a>,
    pub note_samples: u64,
    pub note_current_sample: u64,
    pub done: bool,
    pub seconds_per_beat: f64,

    pub ramp: f64,
    pub volume: f64,

    // Used to generate the current sample
    pub frequency: Option<f64>,
}

impl<'a> VoiceIterator<'a> {
    pub fn new(voice: &'a Voice, seconds_per_beat: f64) -> VoiceIterator<'a> {
        VoiceIterator {
            instrument: voice.instrument,
            envelope: &voice.envelope,
            note_iterator: Box::new(voice.notes.iter()),
            note_samples: 0,
            note_current_sample: 0,
            done: false,
            seconds_per_beat,
            volume: 255.0 / voice.volume as f64,
            ramp: 0.0,
            frequency: None,
        }
    }
}

impl<'a> Iterator for VoiceIterator<'a> {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.done {
            return None;
        }

        // Need next note
        if self.note_current_sample >= self.note_samples {
            self.note_current_sample = 0;

            match self.note_iterator.next() {
                Some(note) => {
                    self.note_samples =
                        (self.seconds_per_beat * SAMPLE_RATE) as u64 * (note.length as u64 + 1);
                    self.frequency = note.frequency();
                }
                None => {
                    self.done = true;
                    return None;
                }
            }
        }

        self.note_current_sample += 1;

        let mut sample = 0.0;

        if let Some(frequency) = self.frequency {
            self.ramp += frequency;

            while self.ramp >= SAMPLE_RATE {
                self.ramp -= SAMPLE_RATE;
            }

            sample += self.instrument.sample(self.ramp / SAMPLE_RATE)
                * self.volume
                * self.envelope.amplitude_at_time(
                    self.note_samples as f64 / SAMPLE_RATE,
                    self.note_current_sample as f64 / SAMPLE_RATE,
                );
        } else {
            self.ramp = 0.0;
        }

        Some(sample)
    }
}
