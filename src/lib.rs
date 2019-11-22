// Each voice will maintain its own ramp for consistency of transition between notes.  Might make
// the sampling a trait, and extend the traits by other traits, like trait Sampler, trait Sawtooth:
// Sampler, but then I'll have to use dynamic typing everywhere.  I can also just make an
// "instrument" structure, and a voice will have an instrument and notes
pub struct Note {
    pub pitch: u8,
    pub length: u8,
}

pub struct Song {
    pub bps: f32,
    pub base_frequency: f32,
    pub notes: Vec<Note>,
}

impl Song {
    /** Render the given song into 48KHz 64-bit floating point PCM.
     */
    pub fn render(&self) -> Vec<f64> {
        let sample_rate = 48_000f64;
        let seconds_per_beat = 1.0 / self.bps;

        let mut output = Vec::new();

        let mut ramp = 0f64;

        let multiplier = 2.0f64.powf(1.0 / 12.0);

        for note in &self.notes {
            // Sample loop, sawtooth
            for _ in 0..((note.length as u32 + 1) * (seconds_per_beat as f64 * sample_rate) as u32) {
                output.push(
                    if note.pitch == 0 {
                        ramp = 0.0;
                        0.0
                    }
                    else {
                        let note_frequency = self.base_frequency as f64 * multiplier.powi(note.pitch as i32 - 1);
                        ramp += note_frequency;
                        while ramp >= sample_rate {
                            ramp -= sample_rate;
                        }
                        ramp / sample_rate * 2.0 - 1.0
                    }
                );
            }
        }

        output
    }
}
