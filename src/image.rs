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

        let mut data: Vec<Superpixel> = Vec::new();

        for chunk in to_encode.chunks(3) {
            // Make sure we always reserve 4 pixels at least
            let mut chunk: Vec<u8> = chunk.into_iter().copied().collect();

            // Adding 0s on the end doesn't matter because of the length prefix.
            chunk.resize(3, 0);
            data.push(Superpixel::Value((chunk[0] & 0b11111100) >> 2));
            data.push(Superpixel::Value(
                ((chunk[0] & 0b00000011) << 4) | ((chunk[1] & 0b11110000) >> 4),
            ));
            data.push(Superpixel::Value(
                ((chunk[1] & 0b00001111) << 2) | ((chunk[2] & 0b11000000) >> 6),
            ));
            data.push(Superpixel::Value(chunk[2] & 0b00111111));
        }

        // needed width
        let width = ((data.len() + 9) as f32).sqrt().ceil() as u8;

        // This is stupidly inefficient.  Each insert will push everything over each time.

        // Insert target pattern
        // BWB
        // WWB
        // BBB
        data.insert(0, Superpixel::Black);
        data.insert(1, Superpixel::White);
        data.insert(2, Superpixel::Black);
        data.insert(width as usize, Superpixel::White);
        data.insert(width as usize + 1, Superpixel::White);
        data.insert(width as usize + 2, Superpixel::Black);
        data.insert(width as usize * 2, Superpixel::Black);
        data.insert(width as usize * 2 + 1, Superpixel::Black);
        data.insert(width as usize * 2 + 2, Superpixel::Black);
        data.resize(width as usize * width as usize, Superpixel::Ignore);
        Payload { width, data }
    }

    /// Takes in data as raw superpixels and width, checking the target and vector size.
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

    pub fn get_superpixel(&self, x: u8, y: u8) -> &Superpixel {
        let index = x as usize + y as usize * self.width as usize;
        if index >= self.data.len() {
            self.data.last().unwrap()
        } else {
            &self.data[index]
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
        if self.r < 20 && self.g < 20 && self.b < 20 {
            Superpixel::Black
        } else if self.r > 235 && self.g > 235 && self.b > 235 {
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
                // Prevent superpixel wrapping
                (payload.width - 1).min((x / superpixel_width) as u8),
                (payload.width - 1).min((y / superpixel_height) as u8),
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
                    return Ok(i as u32);
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
                    return Ok(i as u32);
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
                for sub_y in 0..superpixel_width {
                    for sub_x in 0..superpixel_height {
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
}
