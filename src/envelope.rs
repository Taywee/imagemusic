use serde::de::{self, Error};
use serde::ser::{self, SerializeTuple};

use std::cell::RefCell;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Point {
    /// Stop point in seconds.  May be positive to be distance from the beginning and negative to
    /// be distance from the end.  If these overlap, the envelope will change shape in ways that
    /// may be unpredictable, because they will reorder automatically.
    pub stop: f32,

    /// Height of the wave at this stop
    pub amplitude: f32,
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

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("A pair of tuples")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let stop: Option<i8> = seq.next_element()?;
        let stop = stop.ok_or_else(|| A::Error::invalid_length(0, &self))?;
        let amplitude: Option<u8> = seq.next_element()?;
        let amplitude = amplitude.ok_or_else(|| A::Error::invalid_length(1, &self))?;
        let stop = stop as f32 / 100.0;
        let amplitude = amplitude as f32 / 255.0;
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

#[derive(Debug, Clone, PartialEq)]
pub struct Envelope {
    pub points: Vec<Point>,

    // Specific points for note
    note_points: RefCell<Vec<Point>>,
}

impl ser::Serialize for Envelope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        use ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.points.len()))?;
        for point in &self.points {
            seq.serialize_element(point)?;
        }
        seq.end()
    }
}

struct EnvelopeVisitor;

impl<'de> de::Visitor<'de> for EnvelopeVisitor {
    type Value = Envelope;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("A sequence of points")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let mut points = match seq.size_hint() {
            Some(size) => Vec::with_capacity(size),
            None => Vec::new(),
        };
        while let Some(point) = seq.next_element()? {
            points.push(point);
        }
        Ok(Envelope {
            points,
            note_points: RefCell::new(Vec::new()),
        })
    }
}

impl<'de> de::Deserialize<'de> for Envelope {
    fn deserialize<D>(deserializer: D) -> Result<Envelope, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_seq(EnvelopeVisitor)
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            points: vec![
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
            ],
            note_points: RefCell::new(Vec::new()),
        }
    }
}

#[inline]
fn lerp(x: f32, a: (f32, f32), b: (f32, f32)) -> f32 {
    a.1 + (x - a.0) * (b.1 - a.1) / (b.0 - a.0)
}

impl Envelope {
    pub fn prepare_note(&self, note_length: f32) {
        if self.points.len() == 1 {
            return;
        }

        // Envelope points are made absolute here (all to time from beginning).  Also remove points
        // with a stop outside the note's range.
        let mut points: Vec<Point> = self
            .points
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

        let mut note_points = self.note_points.borrow_mut();
        *note_points = points;
    }

    pub fn amplitude_at_time(&self, time_point: f32) -> f32 {
        if self.points.len() == 1 {
            return self.points.get(0).unwrap().amplitude;
        }

        let points = self.note_points.borrow();

        // Can probably make all of this much nicer.
        if points.first().unwrap().stop >= time_point {
            points.first().unwrap().amplitude
        } else if points.last().unwrap().stop <= time_point {
            points.last().unwrap().amplitude
        } else {
            for window in points.windows(2) {
                let (first, second) = (window[0], window[1]);
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
