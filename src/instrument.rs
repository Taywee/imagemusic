use std::f64;

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
        (((ramp - 0.25).abs() - 0.5).abs() - 0.25
         ) * 4.0
    }
}

impl dyn Instrument {
    pub fn from_id(id: u32) -> Result<Box<dyn Instrument>, ()> {
        match id {
            0 => Ok(Box::new(Sawtooth)),
            1 => Ok(Box::new(Square)),
            2 => Ok(Box::new(Triangle)),
            3 => Ok(Box::new(Sine)),
            _ => Err(()),
        }
    }
}
