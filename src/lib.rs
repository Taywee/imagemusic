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
pub struct Square;
pub struct Triangle;

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

impl Instrument for Square {
    fn sample(&self, ramp: f64) -> f64 {
        if ramp >= 0.5 {
            1.0
        } else {
            -1.0
        }
    }
}

impl Instrument for Triangle {
    fn sample(&self, ramp: f64) -> f64 {
        (((ramp - 0.25).abs() - 0.5).abs() - 0.25) * 4.0
    }
}

pub struct Note {
    pub pitch: i8,
    pub length: u8,
}

pub struct EnvelopePoint {
    /// Height of the wave at this stop
    pub amplitude: f64,

    /// Stop point in seconds.  May be positive to be distance from the beginning and negative to
    /// be distance from the end.  If these overlap, the envelope will change shape in ways that
    /// may be unpredictable, because they will reorder automatically.
    pub stop: f64,
}

pub struct Envelope {
    pub points: Vec<EnvelopePoint>,
}

fn lerp(x: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    a.1 + (x - a.0) * (b.1 - a.1) / (b.0 - a.0)
}

impl Envelope {
    fn amplitude_at_time(&self, note_length: f64, time_point: f64) -> f64 {
        if self.points.len() == 0 {
            // This should never happen, as an empty envelope will be prevented
            panic!("An envelope should never be empty.");
        }

        if self.points.len() == 1 {
            return self.points.get(0).unwrap().amplitude;
        }

        // Envelope points are made absolute here (all to time from beginning)
        let mut points: Vec<EnvelopePoint> = self.points.iter().map(|point| {
            let stop = 
                if point.stop < 0.0 {
                    // Negative, so will subtract
                    note_length + point.stop
                } else {
                    point.stop
                };

            EnvelopePoint{
                amplitude: point.amplitude,
                stop,
            }
        }).collect();

        // Sort the points
        points.sort_by(|a, b| a.stop.partial_cmp(&b.stop).unwrap());

        // Can probably make all of this much nicer.
        if points.first().unwrap().stop >= time_point {
            points.first().unwrap().amplitude
        } else if points.last().unwrap().stop <= time_point {
            points.last().unwrap().amplitude
        } else {
            for i in 1..(points.len() - 1) {
                let first = points.get(i - 1).unwrap();
                let second = points.get(i).unwrap();
                if first.stop <= time_point && time_point <= second.stop {
                    return lerp(time_point, (first.stop, first.amplitude), (second.stop, second.amplitude));
                }
            }

            // on last branch: last two points
            let last_index = points.len() - 1;
            let first = points.get(last_index - 1).unwrap();
            let second = points.get(last_index).unwrap();
            lerp(time_point, (first.stop, first.amplitude), (second.stop, second.amplitude))
        }
    }
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
    instrument: &'a dyn Instrument,
    envelope: &'a Envelope,
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
    volume: f64,
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

pub struct Song {
    pub bps: f64,
    pub voices: Vec<Voice>,
    pub sample_rate: f64,
}

pub struct SongIterator<'a> {
    pub voice_iterators: Vec<VoiceIterator<'a>>,
    pub volume_modifier: f64,
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
    pub fn voice_iterators(&mut self) -> Vec<VoiceIterator> {
        self.voices.iter()
            .map(|voice| {
                VoiceIterator {
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
                }
            })
        .collect()
    }

    pub fn iter(&mut self) -> SongIterator {
        let voice_iterators = self.voice_iterators();
        let volume_modifier = 1.0 / (voice_iterators.len() as f64);
        SongIterator{
            voice_iterators,
            volume_modifier,
        }
    }
}
