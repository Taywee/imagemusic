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

/// Superpixel affinity, determines whether a superpixel is black, white, or the value of the pixel
/// itself.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Superpixel {
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
            chunk.resize(3, 0);
            data.push(Superpixel::Value((chunk[0] & 0b11111100) >> 2));
            data.push(Superpixel::Value(((chunk[0] & 0b00000011) << 4) | ((chunk[1] & 0b11110000) >> 4)));
            data.push(Superpixel::Value(((chunk[1] & 0b00001111) << 2) | ((chunk[2] & 0b11000000) >> 6)));
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
        Payload {
            width,
            data,
        }
    }

    pub fn get_superpixel(&self, x: u16, y: u16) -> &Superpixel {
        &self.data[x as usize + y as usize * self.width as usize]
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
    pixels: Vec<Pixel>
}
/// Round a value to an affinity sixteenth
/// Affinity must be [0, 3], or this will panic.
fn round_to_affinity(affinity: u8, input: u8) -> u8 {
    if affinity > 3 {
        panic!("Affinity must be less than 4");
    }
    let offset = 8 + 16 * affinity;

    // (Distance from value, value)
    (0u8..4u8).map(|quadrant| {
        let value = quadrant * 64 + offset;
        let distance = value.max(input) - value.min(input);
        (distance, value)
    }).min_by_key(|(distance, _)| *distance).unwrap().1
}

impl Image {
    /// Panics if pixels length does not match dimensions.
    pub fn new<P: Into<Vec<Pixel>>>(dimensions: (u32, u32), pixels: P) -> Self {
        let pixels = pixels.into();
        if pixels.len() != dimensions.0 as usize * dimensions.1 as usize {
            panic!("Pixels must match in size");
        }
        Image {
            dimensions,
            pixels,
        }
    }
    
    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    /// Bake a payload into this image.
    pub fn bake_payload(&mut self, payload: &Payload) {
        let superpixel_width = self.dimensions.0 / payload.width as u32;
        let superpixel_height = self.dimensions.1 / payload.width as u32;

        for (i, pixel) in self.pixels.iter_mut().enumerate() {
            let i = i as u32;
            let x = i % self.dimensions.0;
            let y = i % self.dimensions.0;

            let superpixel = payload.get_superpixel((x / superpixel_width) as u16, (y / superpixel_height) as u16);
            match superpixel {
                Superpixel::Black => *pixel = Pixel {
                    r: u8::MIN,
                    g: u8::MIN,
                    b: u8::MIN,
                    a: u8::MAX,
                },
                Superpixel::White => *pixel = Pixel {
                    r: u8::MAX,
                    g: u8::MAX,
                    b: u8::MAX,
                    a: u8::MAX,
                },
                Superpixel::Value(value) => {
                    *pixel = Pixel {
                        r: round_to_affinity((value & 0b00110000) >> 4, pixel.r),
                        g: round_to_affinity((value & 0b00001100) >> 2, pixel.g),
                        b: round_to_affinity(value & 0b00000011, pixel.b),
                        a: pixel.a,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn affinity_rounding() {
        assert_eq!(round_to_affinity(0, 0), 8);
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
        assert_eq!(round_to_affinity(3, 255), 248);
    }
}
