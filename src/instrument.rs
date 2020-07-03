use serde::{Deserialize, Serialize};
use std::f32;

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instrument {
    Sawtooth,
    Sine,
    Square,
    Triangle,
}

impl Default for Instrument {
    fn default() -> Self {
        Instrument::Square
    }
}

impl Instrument {
    pub fn sample(self, ramp: f32) -> f32 {
        match self {
            Instrument::Sawtooth => ramp * 2.0 - 1.0,
            Instrument::Sine => (ramp * f32::consts::PI * 2.0).sin(),
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
        }
    }
}
