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
    pub fn amplitude_at_time(&self, note_length: f64, time_point: f64) -> f64 {
        if self.points.len() == 0 {
            // This should never happen, as an empty envelope will be prevented
            panic!("An envelope should never be empty.");
        }

        if self.points.len() == 1 {
            return self.points.get(0).unwrap().amplitude;
        }

        // Envelope points are made absolute here (all to time from beginning)
        let mut points: Vec<EnvelopePoint> = self
            .points
            .iter()
            .map(|point| {
                let stop = if point.stop < 0.0 {
                    // Negative, so will subtract
                    note_length + point.stop
                } else {
                    point.stop
                };

                EnvelopePoint {
                    amplitude: point.amplitude,
                    stop,
                }
            })
            .collect();

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
                    return lerp(
                        time_point,
                        (first.stop, first.amplitude),
                        (second.stop, second.amplitude),
                    );
                }
            }

            // on last branch: last two points
            let last_index = points.len() - 1;
            let first = points.get(last_index - 1).unwrap();
            let second = points.get(last_index).unwrap();
            lerp(
                time_point,
                (first.stop, first.amplitude),
                (second.stop, second.amplitude),
            )
        }
    }
}
