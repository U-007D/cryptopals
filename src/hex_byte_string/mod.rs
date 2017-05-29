mod consts;
use self::consts::msgs;
use super::fluent_validator::{FluentValidator, Error};
pub mod validator;

use std;
use std::fmt;
use byte_buffer::ByteBuffer;
use std::str::Chars;
use owned_chars::{OwnedChars, OwnedCharsExt};

type Result<T> = std::result::Result<T, Error>;
pub type CharPair = (char, char);

#[cfg(test)] mod unit_tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HexByteString(String);

impl HexByteString {
    pub fn new<T: AsRef<str>>(value: T) -> Result<HexByteString> {
        Ok(HexByteString(value.as_ref().validate::<HexByteString>()?.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn iter(&self) -> Iter {
        Iter { iter: self.0.chars() }
    }
}

impl fmt::Display for HexByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HexByteString> for String {
    fn from(hex_byte_string: HexByteString) -> Self {
        hex_byte_string.0
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = CharPair;

    fn next(&mut self) -> Option<CharPair> {
        match (self.iter.next(), self.iter.next()) {
            (Some(first), Some(last)) => Some((first, last)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct IntoIter {
    iter: OwnedChars,
}

impl IntoIterator for HexByteString {
    type Item = CharPair;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter { iter: self.0.into_chars() }
    }
}

impl Iterator for IntoIter {
    type Item = CharPair;

    fn next(&mut self) -> Option<CharPair> {
        match (self.iter.next(), self.iter.next()) {
            (Some(first), Some(last)) => Some((first, last)),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}
