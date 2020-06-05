use crate::envelope::Envelope;
use crate::error::GenerateSamplesError;
use crate::instrument::Instrument;
use std::borrow::Borrow;

// Can not do this because powf is not a const function
//const MULTIPLIER: f64 = 2.0f64.powf(1.0 / 12.0);
const MULTIPLIER: f64 = 1.0594630943592953098431053149397484958171844482421875;

#[derive(Debug)]
pub struct Note {
    pub pitches: Vec<i8>,
    pub length: u8,
}

impl Note {
    pub fn len(&self) -> usize {
        self.pitches.len()
    }
}

#[derive(Debug)]
pub struct Voice {
    pub volume: f64,
    pub start_frequency: f64,
    pub instrument: Box<dyn Instrument>,
    pub notes: Vec<Note>,
    pub envelope: Envelope,
}

impl Voice {
    /**
     * The first largest chord in the voice.
     */
    pub fn largest_chord<'a>(&'a self) -> Option<&'a Note> {
        self.notes.iter().max_by_key(|note| note.len())
    }
}

/**
 * Iterate through all the samples in a voice.
 *
 * This does not return a bound value.
 * Its output should be scaled based on the voice's chord size.
 */
pub struct VoiceIterator<'a> {
    pub instrument: &'a dyn Instrument,
    pub envelope: &'a Envelope,
    pub note_iterator: Box<dyn Iterator<Item = &'a Note> + 'a>,
    pub note_samples: u64,
    pub note_current_sample: u64,
    pub done: bool,
    pub seconds_per_beat: f64,
    pub sample_rate: f64,

    // Used to calculate the next note
    pub frequency: f64,
    pub ramps: Vec<f64>,
    pub volume: f64,

    // Used to generate the current sample
    pub frequencies: Vec<Option<f64>>,

    pub largest_chord_size: usize,
}

impl<'a> VoiceIterator<'a> {
    pub fn new(
        voice: &'a Voice,
        seconds_per_beat: f64,
        sample_rate: f64,
    ) -> Result<VoiceIterator<'a>, GenerateSamplesError> {
        let largest_chord_size = voice
            .largest_chord()
            .ok_or(GenerateSamplesError::EmptyVoice)?
            .pitches
            .len();
        Ok(VoiceIterator {
            instrument: voice.instrument.borrow(),
            envelope: &voice.envelope,
            note_iterator: Box::new(voice.notes.iter()),
            note_samples: 0,
            note_current_sample: 0,
            done: false,
            seconds_per_beat,
            sample_rate: sample_rate,
            frequency: voice.start_frequency,
            volume: voice.volume,
            ramps: vec![0.0; largest_chord_size],
            frequencies: vec![None; largest_chord_size],
            largest_chord_size,
        })
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

            self.frequencies.clear();

            match self.note_iterator.next() {
                Some(note) => {
                    self.note_samples = (self.seconds_per_beat * self.sample_rate) as u64
                        * (note.length as u64 + 1);
                    // -16 is special rest value, 0 is no change, change is shifting up or down
                    for pitch in &note.pitches {
                        match pitch {
                            -16 => {
                                self.frequencies.push(None);
                            }
                            &pitch => {
                                if pitch != 0 {
                                    self.frequency *= MULTIPLIER.powi(pitch as i32);
                                }
                                self.frequencies.push(Some(self.frequency));
                            }
                        }
                    }
                }
                None => {
                    self.done = true;
                    return None;
                }
            }
        }

        // Fill with Nones if necessary
        self.frequencies.resize(self.largest_chord_size, None);

        self.note_current_sample += 1;

        let mut sample = 0.0;

        for (ramp, frequency) in self.ramps.iter_mut().zip(&self.frequencies) {
            if let Some(frequency) = frequency {
                *ramp += frequency;

                while *ramp >= self.sample_rate {
                    *ramp -= self.sample_rate;
                }

                sample += self.instrument.borrow().sample(*ramp / self.sample_rate)
                    * self.volume
                    * self.envelope.amplitude_at_time(
                        self.note_samples as f64 / self.sample_rate,
                        self.note_current_sample as f64 / self.sample_rate,
                    );
            } else {
                *ramp = 0.0;
            }
        }
        Some(sample)
    }
}
