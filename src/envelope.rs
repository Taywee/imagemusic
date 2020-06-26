use serde::de::{self, Error};
use serde::ser::{self, SerializeTuple};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Point {
    /// Stop point in seconds.  May be positive to be distance from the beginning and negative to
    /// be distance from the end.  If these overlap, the envelope will change shape in ways that
    /// may be unpredictable, because they will reorder automatically.
    pub stop: f64,

    /// Height of the wave at this stop
    pub amplitude: f64,
}

impl ser::Serialize for Point {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&((self.stop * 100.0) as i8))?;
        tup.serialize_element(&((self.amplitude * 255.0) as u8))?;
        tup.end()
    }
}

struct PointVisitor;

impl<'de> de::Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A pair of tuples")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let stop: Option<i8> = seq.next_element()?;
        let stop = stop.ok_or_else(|| A::Error::invalid_length(1, &self))?;
        let amplitude: Option<u8> = seq.next_element()?;
        let amplitude = amplitude.ok_or_else(|| A::Error::invalid_length(0, &self))?;
        let stop = stop as f64 / 100.0;
        let amplitude = amplitude as f64 / 255.0;
        Ok(Point { stop, amplitude })
    }
}

impl<'de> de::Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Point, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, PointVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Envelope(pub Vec<Point>);

impl Default for Envelope {
    fn default() -> Self {
        Envelope(vec![
            Point {
                stop: 0.0,
                amplitude: 0.0,
            },
            Point {
                stop: 0.05,
                amplitude: 1.0,
            },
            Point {
                stop: -0.05,
                amplitude: 0.8,
            },
            Point {
                stop: -0.01,
                amplitude: 0.0,
            },
        ])
    }
}

#[inline]
fn lerp(x: f64, a: (f64, f64), b: (f64, f64)) -> f64 {
    a.1 + (x - a.0) * (b.1 - a.1) / (b.0 - a.0)
}

impl Envelope {
    pub fn amplitude_at_time(&self, note_length: f64, time_point: f64) -> f64 {
        if self.0.len() == 0 {
            // This should never happen, as an empty envelope will be prevented
            panic!("An envelope should never be empty.");
        }

        if self.0.len() == 1 {
            return self.0.get(0).unwrap().amplitude;
        }

        // Envelope points are made absolute here (all to time from beginning).  Also remove points
        // with a stop outside the note's range.
        let mut points: Vec<Point> = self
            .0
            .iter()
            .filter_map(|point| {
                let stop = if point.stop < 0.0 {
                    // Negative, so will subtract
                    note_length + point.stop
                } else {
                    point.stop
                };

                Some(Point {
                    amplitude: point.amplitude,
                    stop,
                })
                .filter(|p| (0.0..=note_length).contains(&p.stop))
            })
            .collect();

        let mut lastmax = -1.0;
        // Remove out-of-order stops.  This may cause buggy results, but less buggy results than
        // simple sorting.
        //points.sort_by(|a, b| a.stop.partial_cmp(&b.stop).unwrap());
        points.retain(move |point| {
            if point.stop > lastmax {
                lastmax = point.stop;
                true
            } else {
                false
            }
        });

        // Can probably make all of this much nicer.
        if points.first().unwrap().stop >= time_point {
            points.first().unwrap().amplitude
        } else if points.last().unwrap().stop <= time_point {
            points.last().unwrap().amplitude
        } else {
            for i in 1..points.len() {
                let first = points.get(i - 1).unwrap();
                let second = points.get(i).unwrap();
                if first.stop <= time_point && time_point <= second.stop {
                    return lerp(
                        time_point,
                        (first.stop, first.amplitude),
                        (second.stop, second.amplitude),
                    );
                }
            }

            unreachable!();
        }
    }
}
