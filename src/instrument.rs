use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f64;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instrument {
    Sawtooth,
    Sine,
    Square,
    Triangle,
    Noise,
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument::Square
    }
}

impl Instrument {
    pub fn sample(self, ramp: f64) -> f64 {
        match self {
            Instrument::Sawtooth => ramp * 2.0 - 1.0,
            Instrument::Sine => (ramp * f64::consts::PI * 2.0).sin(),
            Instrument::Square => {
                if ramp >= 0.5 {
                    1.0
                } else {
                    -1.0
                }
            }
            // Builds this pattern:
            //  /\
            // /  \
            //     \  /
            //      \/
            Instrument::Triangle => (((ramp - 0.25).abs() - 0.5).abs() - 0.25) * 4.0,
            Instrument::Noise => {
                let mut rng = rand::thread_rng();
                rng.gen_range(-1.0, 1.0)
            }
        }
    }
}
