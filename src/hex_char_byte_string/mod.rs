mod consts;
use self::consts::msgs;
use super::fluent_validator::{FluentValidator, Error};
pub mod validator;

use std;
use std::fmt;
use byte_buffer::ByteBuffer;
use std::str::Chars;
use owned_chars::{OwnedChars, OwnedCharsExt};
use hex_char::HexChar;

type Result<T> = std::result::Result<T, Error>;
pub type HexCharPair = (HexChar, HexChar);

#[cfg(test)] mod unit_tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HexCharByteString(String);

impl HexCharByteString {
    pub fn new<T: AsRef<str>>(value: T) -> Result<HexCharByteString> {
        Ok(HexCharByteString(value.as_ref().validate::<HexCharByteString>()?.to_string()))
    }

    pub fn iter(&self) -> Iter {
        Iter { iter: self.0.chars() }
    }

    pub fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for HexCharByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HexCharByteString> for String {
    fn from(hex_byte_string: HexCharByteString) -> Self {
        hex_byte_string.0
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = HexCharPair;

    //TODO: DRY Iterator for Iter's & IntoIter's next() implementations (see below)
    fn next(&mut self) -> Option<HexCharPair> {
        match (self.iter.next(), self.iter.next()) {
            (Some(first), Some(last)) => Some((HexChar::new(first).expect(msgs::ERR_UNREACHABLE),
                                              HexChar::new(last).expect(msgs::ERR_UNREACHABLE))),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct IntoIter {
    iter: OwnedChars,
}

impl IntoIterator for HexCharByteString {
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
            (Some(first), Some(last)) => Some((HexChar::new(first).expect(msgs::ERR_UNREACHABLE),
                                              HexChar::new(last).expect(msgs::ERR_UNREACHABLE))),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

impl<'a> IntoIterator for &'a HexCharByteString {
    type Item = HexCharPair;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl From<ByteBuffer> for HexCharByteString {
    fn from(byte_buffer: ByteBuffer) -> Self {
        HexCharByteString::new("0123456789abcdef").ok().unwrap()
    }
}
