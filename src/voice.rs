use crate::envelope::Envelope;
use crate::instrument::Instrument;
use std::borrow::Borrow;

// Can not do this because powf is not a const function
//const MULTIPLIER: f64 = 2.0f64.powf(1.0 / 12.0);
const MULTIPLIER: f64 = 1.0594630943592953098431053149397484958171844482421875;

pub struct Note {
    pub pitch: i8,
    pub length: u8,
}

pub struct Voice {
    pub volume: f64,
    pub start_frequency: f64,
    pub instrument: Box<dyn Instrument>,
    pub notes: Vec<Note>,
    pub envelope: Envelope,
}

/** Iterate through all the samples in a voice
*/
pub struct VoiceIterator<'a> {
    pub instrument: &'a dyn Instrument,
    pub envelope: &'a Envelope,
    pub note_iterator: Box<dyn Iterator<Item = &'a Note> + 'a>,
    pub current_note: Option<&'a Note>,
    pub note_samples: u64,
    pub note_current_sample: u64,
    pub done: bool,
    pub resting: bool,
    pub seconds_per_beat: f64,
    pub sample_rate: f64,
    pub frequency: f64,
    pub ramp: f64,
    pub volume: f64,
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

            self.current_note = match self.note_iterator.next() {
                Some(note) => {
                    self.note_samples = (self.seconds_per_beat * self.sample_rate) as u64 * (note.length as u64 + 1);
                    // -16 is special rest value, 0 is no change, change is shifting up or down
                    match note.pitch {
                        -16 => {
                            self.resting = true;
                            self.ramp = 0.0;
                        },
                        pitch => {
                            self.resting = false;
                            if pitch != 0 {
                                self.frequency *= MULTIPLIER.powi(pitch as i32);
                            }
                        },
                    }
                    Some(note)
                }
                None => {
                    self.done = true;
                    return None;
                },
            }
        }

        self.note_current_sample += 1;

        if self.resting {
            Some(0.0)
        } else {
            self.ramp += self.frequency;
            while self.ramp >= self.sample_rate {
                self.ramp -= self.sample_rate;
            }
            Some(
                self.instrument.borrow().sample(self.ramp / self.sample_rate)
                * self.volume
                * self.envelope.amplitude_at_time(
                    self.note_samples as f64 / self.sample_rate,
                    self.note_current_sample as f64 / self.sample_rate,
                    )
                )
        }
    }
}
