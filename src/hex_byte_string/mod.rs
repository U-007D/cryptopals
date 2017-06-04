use std;
use std::fmt;
use std::str::Chars;

use owned_chars::{OwnedChars, OwnedCharsExt};

mod consts;
pub mod validator;
mod hex_char;
mod hex_value;

use consts::*;
pub use self::consts::*;
use fluent_validator::{FluentValidator, Error};
use byte_buffer::ByteBuffer;
use self::hex_char::HexChar;
use self::hex_value::HexValueExt;

type Result<T> = std::result::Result<T, Error>;
pub type HexCharPair = (HexChar, HexChar);

#[cfg(test)] mod unit_tests;

/// Represents a series of bytes in hexadecimal.  Must contain an even number of characters to properly represent bytes.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HexByteString(String);

impl HexByteString {
    pub fn new<T: AsRef<str>>(value: T) -> Result<HexByteString> {
        Ok(HexByteString(value.as_ref().validate::<HexByteString>()?.to_string()))
    }

    pub fn iter(&self) -> Iter {
        Iter { iter: self.0.chars() }
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for HexByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Provides Into<HexByteString> (not used in this project at this point)
impl From<HexByteString> for String {
    fn from(hex_byte_string: HexByteString) -> Self {
        hex_byte_string.0
    }
}

/// State object for Iterator (non-owning)
#[derive(Debug)]
pub struct Iter<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = HexCharPair;

    //TODO: DRY Iterator for Iter's & IntoIter's next() implementations (see below)
    fn next(&mut self) -> Option<HexCharPair> {
        match (self.iter.next(), self.iter.next()) {
            (Some(first), Some(last)) => Some((HexChar::new(first).expect(ERR_UNREACHABLE),
                                               HexChar::new(last).expect(ERR_UNREACHABLE))),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

/// State object for IntoIterator (owning)
#[derive(Debug)]
pub struct IntoIter {
    iter: OwnedChars,
}

impl IntoIterator for HexByteString {
    type Item = HexCharPair;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter { iter: self.0.into_chars() }
    }
}

impl Iterator for IntoIter {
    type Item = HexCharPair;

    //TODO: DRY Iterator for Iter's & IntoIter's next() implementations (see above)
    fn next(&mut self) -> Option<HexCharPair> {
        match (self.iter.next(), self.iter.next()) {
            (Some(first), Some(last)) => Some((HexChar::new(first).expect(ERR_UNREACHABLE),
                                               HexChar::new(last).expect(ERR_UNREACHABLE))),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

/// Non-owning IntoIterator implementation (DRY--based on Iterator).  Principal benefit is enabling non-consuming for
/// loops (eg. `for item in &collection {...}` where `collection` remains available after the loop).
/// All HexByteString iterators return a pair of `char`s because a pair of hexadecimal digits are required to represent
/// a byte.  This also substatially reduces the amount of state required for anyone iterating over HexByteString, as the
/// state of which byte is being iterated is centralized into the type's iterator.
impl<'a> IntoIterator for &'a HexByteString {
    type Item = HexCharPair;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

/// Also provides Into<ByteBuffer> (not used in this project at this time)
impl From<ByteBuffer> for HexByteString {
    fn from(byte_buffer: ByteBuffer) -> Self {
        //TODO: Refactor to be w/o allocations?
        HexByteString::new(byte_buffer.iter()
                .flat_map(|byte| vec![(byte >> BITS_PER_HEX_DIGIT).as_hex_char().expect(ERR_UNREACHABLE),
                                      (byte & U8_MASK_MSN).as_hex_char().expect(ERR_UNREACHABLE)].into_iter())
                .collect::<String>()).ok().expect(ERR_UNREACHABLE)
    }
}
