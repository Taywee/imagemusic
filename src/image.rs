//! Standalone image-handling module for managing encoded image data.
//! This is the canonical image handling, encoding, and decoding module, but this is intentionally
//! segregated and standalone.  It will never depend on other parts of the crate outside of it, and
//! may eventually be separated into its own separate crate.
//!
//! The encoding process is roughly:
//!
//! * Take in a series of bytes to be the payload.
//! * Calculate the size of the square grid that will be needed to encode that payload (+9 for the
//!   size target).
//! * Encode the bytes into an affinity array, with the width specified.

mod error;
pub use error::Error;

use std::collections::HashMap;
use std::convert::TryInto;

/// Superpixel affinity, determines whether a superpixel is black, white, or the value of the pixel
/// itself.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Superpixel {
    Ignore,
    Black,
    White,
    /// 1-64
    Value(u8),
}

/// A payload, representing encoded data ready to bake into an image.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Payload {
    width: u8,
    data: Vec<Superpixel>,
}

impl Payload {
    pub fn new<B: AsRef<[u8]>>(input: B) -> Self {
        let input = input.as_ref();
        if input.len() > u16::MAX as usize {
            panic!("Input can not be more than 65536 bytes long");
        }
        let mut to_encode = Vec::with_capacity(input.len() * 4 / 3 + 10);
        to_encode.extend_from_slice(&(input.len() as u16).to_be_bytes());
        to_encode.extend_from_slice(input);

        // Initial target pattern
        let mut data: Vec<Superpixel> = vec![ Superpixel::Black, Superpixel::White, Superpixel::White, Superpixel::White, Superpixel::Black, Superpixel::Black, Superpixel::Black, Superpixel::Black, Superpixel::Black];

        for chunk in to_encode.chunks(3) {
            // Make sure we always reserve 4 pixels at least
            let mut chunk: Vec<u8> = chunk.into_iter().copied().collect();

            // Adding 0s on the end doesn't matter because of the length prefix.
            chunk.resize(3, 0);
            data.push(Superpixel::Value(chunk[0] >> 2));
            data.push(Superpixel::Value(
                ((chunk[0] & 0b00000011) << 4) | (chunk[1] >> 4),
            ));
            data.push(Superpixel::Value(
                ((chunk[1] & 0b00001111) << 2) | (chunk[2] >> 6),
            ));
            data.push(Superpixel::Value(chunk[2] & 0b00111111));
        }

        // needed width
        let width = (data.len() as f32).sqrt().ceil() as u8;

        // All the extra superpixels should be ignore superpixels
        data.resize(width as usize * width as usize, Superpixel::Ignore);

        let mut payload = Payload { width, data: vec![Superpixel::Ignore; width as usize * width as usize] };

        for (dest, src) in payload.unwrapped_payload_mut().into_iter().zip(data.into_iter()) {
            *dest = src;
        }

        payload
    }

    /// Takes in data as raw superpixels and width, checking the target and vector size.
    /// This is taken in normal row-major order, not the corner spiral.
    fn from_raw<V: Into<Vec<Superpixel>>>(width: u8, data: V) -> Result<Self, Error> {
        let data = data.into();
        if data.len() != (width as usize).pow(2) {
            return Err(Error::InvalidDimensions);
        }

        if data[0] == Superpixel::Black
            && data[1] == Superpixel::White
            && data[2] == Superpixel::Black
            && data[width as usize] == Superpixel::White
            && data[width as usize + 1] == Superpixel::White
            && data[width as usize + 2] == Superpixel::Black
            && data[width as usize * 2] == Superpixel::Black
            && data[width as usize * 2 + 1] == Superpixel::Black
            && data[width as usize * 2 + 2] == Superpixel::Black {
                Ok(Payload {
                    width,
                    data,
                })
        } else {
            Err(Error::NoTargetFound)
        }

    }

    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = &Superpixel>> {
        self.data.chunks(self.width as usize).map(|chunk| chunk.into_iter())
    }

    /// Get the indexed row, always grabbing the last if over.
    fn row(&self, index: usize) -> Vec<&Superpixel> {
        if index >= self.width as usize {
            self.rows().last().unwrap().collect()
        } else {
            self.rows().skip(index).next().unwrap().collect()
        }
    }

    /// Get the indexed row, always grabbing the last if over.
    fn row_mut(&mut self, index: usize) -> Vec<&mut Superpixel> {
        if index >= self.width as usize {
            self.rows_mut().last().unwrap().collect()
        } else {
            self.rows_mut().skip(index).next().unwrap().collect()
        }
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = impl Iterator<Item = &mut Superpixel>> {
        self.data.chunks_mut(self.width as usize).map(|chunk| chunk.into_iter())
    }

    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Superpixel>> {
        (0..(self.width as usize)).map(move |column| {
            self.data.iter().skip(column).step_by(self.width as usize)
        })
    }

    /// Get the column as indexed, always grabbing the last if needed.
    fn column(&self, index: usize) -> Vec<&Superpixel> {
        if index >= self.width as usize {
            self.columns().last().unwrap().collect()
        } else {
            self.columns().skip(index).next().unwrap().collect()
        }
    }

    /// Get the mutable column as indexed, always grabbing the last if needed.
    pub fn column_mut(&mut self, index: usize) -> Vec<&mut Superpixel> {
        let index = if index >= self.width as usize {
            self.width as usize - 1
        } else {
            index
        };
        let output: Vec<_> = self.data.iter_mut().skip(index).step_by(self.width as usize).collect();

        output
    }

    /// Get the indexed superpixel.  When out of bounds, the last superpixel in that direction will
    /// be selected.
    pub fn get_superpixel(&self, x: usize, y: usize) -> &Superpixel {
        let column = self.column(x);
        if y >= column.len() {
            column.last().unwrap()
        } else {
            &column[y]
        }
    }

    /// Get the indexed superpixel.  When out of bounds, the last superpixel in that direction will
    /// be selected.
    pub fn get_superpixel_mut(&mut self, x: usize, y: usize) -> &mut Superpixel {
        let x = x.min(self.width as usize - 1);
        let y = y.min(self.width as usize - 1);
        &mut self.data[y * self.width as usize + x]
    }

    /// The payload spirals down from the top left like this:
    /// 0  1  4  9
    /// 3  2  5  10
    /// 8  7  6  11
    /// 15 14 13 12
    pub fn unwrapped_payload(&self) -> Vec<Superpixel> {
        let mut output = Vec::new();
        for radius in 0..self.width {
            for row in 0..=radius {
                output.push(self.data[row as usize * self.width as usize + radius as usize]);
            }

            for column in (0..radius).rev() {
                output.push(self.data[radius as usize * self.width as usize + column as usize]);
            }
        }
        output
    }

    /// The payload spirals down from the top left like this:
    /// 0  1  4  9
    /// 3  2  5  10
    /// 8  7  6  11
    /// 15 14 13 12
    pub fn unwrapped_payload_mut(&mut self) -> Vec<&mut Superpixel> {
        let mut data_vec: Vec<Option<&mut Superpixel>> = self.data.iter_mut().map(|s| Some(s)).collect();
        let mut output = Vec::new();
        for radius in 0..self.width {
            for row in 0..=radius {
                output.push(data_vec[row as usize * self.width as usize + radius as usize].take().unwrap());
            }

            for column in (0..radius).rev() {
                output.push(data_vec[radius as usize * self.width as usize + column as usize].take().unwrap());
            }
        }
        output
    }

    /// Read the data out of this packed payload
    pub fn data(&self) -> Result<Vec<u8>, Error> {
        let data = self.unwrapped_payload();
        // skip target
        let data: Vec<_> = data[9..].into_iter().collect();

        let data: Vec<u8> = data.into_iter().map(|superpixel| {
            use Superpixel::*;
            match superpixel {
                Ignore => 0,
                Black => 0,
                White => 63,
                Value(value) => value & 0b00111111,
            }
        }).collect();

        let mut output = Vec::with_capacity(data.len());

        for chunk in data.chunks(4) {
            let mut chunk: Vec<u8> = chunk.into_iter().copied().collect();
            // Extra 0s shouldn't really matter, but this simplifies decoding
            chunk.resize(4, 0);

            output.push((chunk[0] << 2) | (chunk[1] >> 4));
            output.push((chunk[1] << 4) | (chunk[2] >> 2));
            output.push((chunk[2] << 6) | chunk[3]);
        }

        let length = u16::from_be_bytes(output[0..2].try_into().unwrap());
        // Remove length
        output.remove(0);
        output.remove(0);
        if length as usize > output.len() {
            Err(Error::InvalidLength{
                encoded: length,
                available: output.len() as u16,
            })
        } else {
            output.resize(length as usize, 0);
            Ok(output)
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Image {
    dimensions: (u32, u32),
    pixels: Vec<Pixel>,
}

/// Round a value to an affinity sixteenth
/// Affinity must be [0, 3], or this will panic.
fn round_to_affinity(affinity: u8, input: u8) -> u8 {
    if affinity > 3 {
        panic!("Affinity must be less than 4");
    }
    let offset = 8 + 16 * affinity;

    // (Distance from value, value)
    let value = (0u8..4u8)
        .map(|quadrant| {
            let value = quadrant * 64 + offset;
            let distance = value.max(input) - value.min(input);
            (distance, value)
        })
        .min_by_key(|(distance, _)| *distance)
        .unwrap()
        .1;

    // Floor and ceil values near edge where possible.  anything below 8 or above 248 in
    // here shouldn't be possible, but this is more semantically clear.
    match value {
        0..=8 => 0,
        248..=255 => 255,
        value => value,
    }
}

/// Find the affinity of a color value.
/// output is always 0..4
fn get_affinity(input: u8) -> u8 {
    match input {
        0..=15 => 0,
        16..=31 => 1,
        32..=47 => 2,
        48..=63 => 3,
        64..=79 => 0,
        80..=95 => 1,
        96..=111 => 2,
        112..=127 => 3,
        128..=143 => 0,
        144..=159 => 1,
        160..=175 => 2,
        176..=191 => 3,
        192..=207 => 0,
        208..=223 => 1,
        224..=239 => 2,
        240..=255 => 3,
    }
}

impl Pixel {
    /// Bake the value into this pixel
    pub fn with_value(self, value: u8) -> Pixel {
        Pixel {
            r: round_to_affinity((value & 0b00110000) >> 4, self.r),
            g: round_to_affinity((value & 0b00001100) >> 2, self.g),
            b: round_to_affinity(value & 0b00000011, self.b),
            // no transparent pixels to prevent optimization from killing our encoded
            // data.
            // Optimization may still kill our encoded data if transparency is
            // stripped.
            a: self.a.max(25),
        }
    }

    /// Get the value encoded in this pixel
    pub fn value(self) -> Superpixel {
        // We can't just return the affinitie'd value and assume black for 0 and white for 63,
        // because 0 may not be black, due to quadrants, and 63 may not be white.
        if self.r < 16 && self.g < 16 && self.b < 16 {
            Superpixel::Black
        } else if self.r > 239 && self.g > 239 && self.b > 239 {
            Superpixel::White
        } else {
            let r = get_affinity(self.r);
            let g = get_affinity(self.g);
            let b = get_affinity(self.b);
            Superpixel::Value((r << 4) | (g << 2) | b)
        }
    }
}

impl Image {
    /// Panics if pixels length does not match dimensions.
    pub fn new<P: Into<Vec<Pixel>>>(dimensions: (u32, u32), pixels: P) -> Self {
        let pixels = pixels.into();
        if pixels.len() != dimensions.0 as usize * dimensions.1 as usize {
            panic!("Pixels must match in size");
        }
        Image { dimensions, pixels }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    /// Bake a payload into this image.
    pub fn bake_payload(&mut self, payload: &Payload) {
        // Width is squared, so we determine the pixel width of each superpixel.  This will almost
        // certainly not be perfect.  In the case that there is remainder, the last superpixel in
        // that dimension will be stretched to the edge of the image.
        let superpixel_width = self.dimensions.0 / payload.width as u32;
        let superpixel_height = self.dimensions.1 / payload.width as u32;

        for (i, pixel) in self.pixels.iter_mut().enumerate() {
            let i = i as u32;
            let x = i % self.dimensions.0;
            let y = i / self.dimensions.0;

            let superpixel = payload.get_superpixel(
                x as usize / superpixel_width as usize,
                y as usize / superpixel_height as usize,
            );
            match superpixel {
                Superpixel::Ignore => (),
                Superpixel::Black => {
                    *pixel = Pixel {
                        r: u8::MIN,
                        g: u8::MIN,
                        b: u8::MIN,
                        a: u8::MAX,
                    }
                }
                Superpixel::White => {
                    *pixel = Pixel {
                        r: u8::MAX,
                        g: u8::MAX,
                        b: u8::MAX,
                        a: u8::MAX,
                    }
                }
                Superpixel::Value(value) => {
                    *pixel = pixel.with_value(*value);
                }
            }
        }
    }

    /// Uses the target to determine width of superpixels
    pub fn superpixel_width(&self) -> Result<u32, Error> {
        let mut first_white = false;

        // Only take the first row
        for (i, pixel) in self.pixels.iter().copied().enumerate().take(self.dimensions.0 as usize) {
            if !first_white {
                // looking for the first white pixel
                if pixel.r > 235 && pixel.b > 235 && pixel.g > 235 {
                    first_white = true;
                }
            } else {
                // looking for the first black pixel after the first white
                if pixel.r < 20 && pixel.b < 20 && pixel.g < 20 {
                    return Ok(i as u32 / 2);
                }
            }
        }

        Err(Error::NoTargetFound)
    }

    /// Uses the target to determine height of superpixels
    pub fn superpixel_height(&self) -> Result<u32, Error> {
        let mut first_white = false;

        // Only take the first column
        for (i, pixel) in self.pixels.iter().step_by(self.dimensions.0 as usize).copied().enumerate() {
            if !first_white {
                // looking for the first white pixel
                if pixel.r > 235 && pixel.b > 235 && pixel.g > 235 {
                    first_white = true;
                }
            } else {
                // looking for the first black pixel after the first white
                if pixel.r < 20 && pixel.b < 20 && pixel.g < 20 {
                    return Ok(i as u32 / 2);
                }
            }
        }

        Err(Error::NoTargetFound)
    }

    /// Read a payload from this image.
    pub fn read_payload(&self) -> Result<Payload, Error> {
        let superpixel_width = self.superpixel_width()?;
        let superpixel_height = self.superpixel_height()?;

        let horizontal_superpixels = self.dimensions.0 / superpixel_width;
        let vertical_superpixels = self.dimensions.1 / superpixel_height;

        if horizontal_superpixels != vertical_superpixels {
            return Err(Error::SuperpixelGridNotSquare);
        }

        let mut superpixels = Vec::new();

        for y in 0..vertical_superpixels {
            for x in 0..horizontal_superpixels {
                // All the pixels in this superpixel
                let mut pixels = Vec::new();
                for sub_y in 0..superpixel_height {
                    for sub_x in 0..superpixel_width {
                        // Total pixel offsets
                        let x_offset = x * superpixel_width + sub_x;
                        let y_offset = y * superpixel_height + sub_y;
                        let pixel_offset = y_offset as usize * self.dimensions.0 as usize + x_offset as usize;
                        pixels.push(self.pixels[pixel_offset].value());
                    }
                }

                // Collect these into counted groups
                let counted = pixels.into_iter()
                    .fold(HashMap::new(), |mut acc, pixel| {
                        *acc.entry(pixel).or_insert(0) += 1;
                        acc
                    });

                // Final value determined by max membership
                let superpixel = counted.into_iter().max_by_key(|(_, count)| *count).unwrap().0;
                superpixels.push(superpixel);
            }
        }

        Ok(Payload::from_raw(horizontal_superpixels as u8, superpixels)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rand::{Rng};

    #[test]
    fn affinity_rounding() {
        assert_eq!(round_to_affinity(0, 0), 0);
        assert_eq!(round_to_affinity(1, 0), 24);
        assert_eq!(round_to_affinity(2, 0), 40);
        assert_eq!(round_to_affinity(3, 0), 56);

        assert_eq!(round_to_affinity(0, 64), 72);
        assert_eq!(round_to_affinity(1, 64), 88);
        assert_eq!(round_to_affinity(2, 64), 40);
        assert_eq!(round_to_affinity(3, 64), 56);

        assert_eq!(round_to_affinity(0, 128), 136);
        assert_eq!(round_to_affinity(1, 128), 152);
        assert_eq!(round_to_affinity(2, 128), 104);
        assert_eq!(round_to_affinity(3, 128), 120);

        assert_eq!(round_to_affinity(0, 192), 200);
        assert_eq!(round_to_affinity(1, 192), 216);
        assert_eq!(round_to_affinity(2, 192), 168);
        assert_eq!(round_to_affinity(3, 192), 184);

        assert_eq!(round_to_affinity(0, 255), 200);
        assert_eq!(round_to_affinity(1, 255), 216);
        assert_eq!(round_to_affinity(2, 255), 232);
        assert_eq!(round_to_affinity(3, 255), 255);
    }

    #[test]
    fn payload_roundtrip() {
        let mut rng = rand::thread_rng();

        let dimensions = (100, 100);
        let mut origin_image = Image::new(dimensions, std::iter::from_fn(||
            Some(Pixel {
                r: rng.gen(),
                g: rng.gen(),
                b: rng.gen(),
                a: rng.gen(),
            })).take(dimensions.0 as usize * dimensions.1 as usize).collect::<Vec<Pixel>>());

        let data: Vec<u8> = (0..1000).map(|_| rng.gen()).collect();
        let payload = Payload::new(&data);

        origin_image.bake_payload(&payload);
        let read_data = origin_image.read_payload().expect("Could not read payload").data().expect("Could not read data");
        assert_eq!(data, read_data);
    }

    #[test]
    fn rows_and_columns() {
        use Superpixel::*;
        let payload = Payload {
            width: 4,
            data: vec![
                Black, White, Black, Value(0),
                White, White, Black, Value(1),
                Black, Black, Black, Value(2),
                Value(6), Value(5), Value(4), Value(3),
            ]
        };

        assert_eq!(vec![Black, White, Black, Value(0)], payload.rows().next().unwrap().copied().collect::<Vec<_>>());
        assert_eq!(vec![White, White, Black, Value(1)], payload.rows().skip(1).next().unwrap().copied().collect::<Vec<_>>());
        assert_eq!(vec![Black, Black, Black, Value(2)], payload.rows().skip(2).next().unwrap().copied().collect::<Vec<_>>());
        assert_eq!(vec![Value(6), Value(5), Value(4), Value(3)], payload.rows().skip(3).next().unwrap().copied().collect::<Vec<_>>());
        assert!(matches!(payload.rows().skip(4).next(), None));

        assert_eq!(vec![Black, White, Black, Value(6)], payload.columns().next().unwrap().copied().collect::<Vec<_>>());
        assert_eq!(vec![White, White, Black, Value(5)], payload.columns().skip(1).next().unwrap().copied().collect::<Vec<_>>());
        assert_eq!(vec![Black, Black, Black, Value(4)], payload.columns().skip(2).next().unwrap().copied().collect::<Vec<_>>());
        assert_eq!(vec![Value(0), Value(1), Value(2), Value(3)], payload.columns().skip(3).next().unwrap().copied().collect::<Vec<_>>());
        assert!(matches!(payload.columns().skip(4).next(), None));
    }

    #[test]
    fn unwrapped_payload() {
        use Superpixel::*;
        let mut payload = Payload {
            width: 4,
            data: vec![
                Value(0), Value(1), Value(2), Value(3),
                Value(4), Value(5), Value(6), Value(7),
                Value(8), Value(9), Value(10), Value(11),
                Value(12), Value(13), Value(14), Value(15),
            ]
        };

        assert_eq!(vec![Value(0),
        Value(1), Value(5), Value(4),
        Value(2), Value(6), Value(10), Value(9), Value(8),
        Value(3), Value(7), Value(11), Value(15), Value(14), Value(13), Value(12),
        ], payload.unwrapped_payload());

        assert_eq!(vec![&Value(0),
        &Value(1), &Value(5), &Value(4),
        &Value(2), &Value(6), &Value(10), &Value(9), &Value(8),
        &Value(3), &Value(7), &Value(11), &Value(15), &Value(14), &Value(13), &Value(12),
        ], payload.unwrapped_payload_mut());
    }
}
