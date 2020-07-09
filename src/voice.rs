use crate::envelope::Envelope;
use crate::instrument::Instrument;
use crate::note::{Note, Notes};
use serde::{Deserialize, Serialize};

fn default_volume() -> u8 {
    u8::MAX
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Voice {
    #[serde(default = "default_volume")]
    pub volume: u8,

    #[serde(default)]
    pub instrument: Instrument,
    pub notes: Notes,

    #[serde(default)]
    pub envelope: Envelope,
}

/**
 * Iterate through all the samples in a voice.
 *
 * This does not return a bound value.
 */
pub struct VoiceIterator<'a> {
    pub instrument: Instrument,
    pub envelope: &'a Envelope,
    pub note_iterator: Box<dyn Iterator<Item = &'a Note> + 'a>,
    pub note_samples: u32,
    pub note_current_sample: u32,
    pub done: bool,
    pub seconds_per_beat: f32,

    pub ramp: f32,
    pub volume: f32,

    // Used to generate the current sample
    pub frequency: Option<f32>,

    pub sample_rate: f32,
    pub inverse_sample_rate: f32,
}

impl<'a> VoiceIterator<'a> {
    pub fn new(voice: &'a Voice, seconds_per_beat: f32, sample_rate: usize) -> VoiceIterator<'a> {
        VoiceIterator {
            instrument: voice.instrument,
            envelope: &voice.envelope,
            note_iterator: Box::new(voice.notes.iter()),
            note_samples: 0,
            note_current_sample: 0,
            done: false,
            seconds_per_beat,
            volume: 255.0 / voice.volume as f32,
            ramp: 0.0,
            frequency: None,
            sample_rate: sample_rate as f32,
            inverse_sample_rate: 1.0 / sample_rate as f32,
        }
    }
}

impl<'a> Iterator for VoiceIterator<'a> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.done {
            return None;
        }

        // Need next note
        if self.note_current_sample >= self.note_samples {
            self.note_current_sample = 0;

            match self.note_iterator.next() {
                Some(note) => {
                    self.ramp = 0.0;
                    self.envelope
                        .prepare_note(self.seconds_per_beat * note.length as f32);
                    self.note_samples =
                        (self.seconds_per_beat * self.sample_rate) as u32 * note.length as u32;
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

            while self.ramp >= self.sample_rate {
                self.ramp -= self.sample_rate;
            }

            sample += self.instrument.sample(self.ramp * self.inverse_sample_rate)
                * self.volume
                * self
                    .envelope
                    .amplitude_at_time(self.note_current_sample as f32 * self.inverse_sample_rate);
        }

        Some(sample)
    }
}
