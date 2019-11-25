use std::borrow::Borrow;
use std::f64;
// Each voice will maintain its own ramp for consistency of transition between notes.  Might make
// the sampling a trait, and extend the traits by other traits, like trait Sampler, trait Sawtooth:
// Sampler, but then I'll have to use dynamic typing everywhere.  I can also just make an
// "instrument" structure, and a voice will have an instrument and notes

// Can not do this because powf is not a const function
//const MULTIPLIER: f64 = 2.0f64.powf(1.0 / 12.0);
const MULTIPLIER: f64 = 1.0594630943592953098431053149397484958171844482421875;

pub trait Instrument {
    /** Get a single sample.
     *
     * `ramp` is the frequency ramp, from 0 to 1.0, with 0 implying the beginning of a single wave
     * and 1.0 implying the end of that wave.
     * 
     * Returns the wave value from -1.0 to 1.0
     */
    fn sample(&self, ramp: f64) -> f64;
}

pub struct Sawtooth;
pub struct Sine;

impl Instrument for Sawtooth {
    fn sample(&self, ramp: f64) -> f64 {
        ramp * 2.0 - 1.0
    }
}

impl Instrument for Sine {
    fn sample(&self, ramp: f64) -> f64 {
        (ramp * f64::consts::PI * 2.0).sin()
    }
}

pub struct Note {
    pub pitch: i8,
    pub length: u8,
}

pub struct Voice {
    pub instrument: Box<dyn Instrument>,
    pub notes: Vec<Note>,
}

/** Iterate through all the samples in a voice
*/
pub struct VoiceIterator<'a> {
    instrument: &'a dyn Instrument,
    note_iterator: Box<dyn Iterator<Item = &'a Note> + 'a>,
    current_note: Option<&'a Note>,
    note_samples: u64,
    note_current_sample: u64,
    done: bool,
    resting: bool,
    seconds_per_beat: f64,
    sample_rate: f64,
    frequency: f64,
    ramp: f64,
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
            Some(self.instrument.borrow().sample(self.ramp / self.sample_rate))
        }
    }
}

pub struct Song {
    pub bps: f64,
    pub base_frequency: f64,
    pub voice: Voice,
    pub sample_rate: f64,
}

impl Song {
    pub fn voice_iterator(&mut self) -> VoiceIterator {
        VoiceIterator {
            instrument: self.voice.instrument.borrow(),
            note_iterator: Box::new(self.voice.notes.iter()),
            current_note: None,
            note_samples: 0,
            note_current_sample: 0,
            done: false,
            resting: false,
            seconds_per_beat: 1.0 / self.bps,
            sample_rate: self.sample_rate,
            frequency: self.base_frequency,
            ramp: 0.0,
        }
    }

    /** Render the given song into 48KHz 64-bit floating point PCM.
    */
    pub fn render(&mut self) -> Vec<f64> {
        self.voice_iterator().collect()
    }
}
