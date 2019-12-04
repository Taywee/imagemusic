pub mod error;
use std::fmt;

/** Trait that describes that a type may be converted from base32 with a standard defined
 * conversion.
 */
pub trait FromBase32 : Sized {
    fn from_base32(source: char) -> Result<Self, error::FromBase32Error>;
}

/** i8 converts signed, from -16 to 15
 */
impl FromBase32 for i8 {
    fn from_base32(source: char) -> Result<Self, error::FromBase32Error> {
        Ok(match source.to_ascii_uppercase() {
            x @ 'A'..='Z' => (x as u8 - 'A' as u8) as i8 - 16,
            x @ '2'..='7' => (x as u8 - '2' as u8) as i8 + 10,
            source => return Err(error::FromBase32Error{source}),
        })
    }
}

/** u8 converts unsigned, from 0 to 31
 */
impl FromBase32 for u8 {
    fn from_base32(source: char) -> Result<Self, error::FromBase32Error> {
        Ok(match source.to_ascii_uppercase() {
            x @ 'A'..='Z' => x as u8 - 'A' as u8,
            x @ '2'..='7' => x as u8 - '2' as u8 + 26,
            source => return Err(error::FromBase32Error{source}),
        })
    }
}

/** Trait that describes that a type may be converted to base32 with a standard defined
 * conversion.
 */
pub trait ToBase32: Sized + fmt::Debug {
    fn to_base32(self) -> Result<char, error::ToBase32Error<Self>>;
}

/** i8 converts signed, from -16 to 15
 */
impl ToBase32 for i8 {
    fn to_base32(self) -> Result<char, error::ToBase32Error<Self>> {
        // Need to do this conversion dance to enable x to work positively or negatively
        Ok(match self {
            x @ -16..=9 => ('A' as u8 as i8 + 16 + x) as u8 as char,
            x @ 10..=16 => ('2' as u8 as i8 + x - 10) as u8 as char,
            source => return Err(error::ToBase32Error{source}),
        })
    }
}

/** u8 converts unsigned, from 0 to 31
 */
impl ToBase32 for u8 {
    fn to_base32(self) -> Result<char, error::ToBase32Error<Self>> {
        Ok(match self {
            x @ 0..=25 => ('A' as u8 + x) as char,
            x @ 26..=31 => ('2' as u8 + x - 26) as char,
            source => return Err(error::ToBase32Error{source}),
        })
    }
}

/** A simple trait for char that gives it a method to decode as a base32 character, just for
 * convenience, like Collect.
 */
pub trait Base32 {
    fn base32_decode<T>(self) -> Result<T, error::FromBase32Error> where T: FromBase32;
}

impl Base32 for char {
    fn base32_decode<T>(self) -> Result<T, error::FromBase32Error> where T: FromBase32 {
        T::from_base32(self)
    }
}
