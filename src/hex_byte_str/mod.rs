use super::consts::ERR_UNREACHABLE;

pub mod validator;

use std;
use std::fmt;
use std::str::Chars;
use hex_char::{HexChar, HexCharPair, FromCharExtTrait};
use owned_chars::{OwnedChars, OwnedCharsExt};
use byte_buffer::ByteBuffer;
use fluent_validator::FluentValidator;

type Result<T> = std::result::Result<T, validator::Error>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HexByteStr<'a>(&'a str);

impl<'a> HexByteStr<'a> {
    pub fn new<T: AsRef<str>>(init_value: T) -> Result<HexByteStr<'a>> {
        Ok(HexByteStr(init_value.as_ref().validate::<HexByteStr>()?))
    }

    pub fn iter(&self) -> Iter {
        Iter { iter: self.0.chars() }
    }
}

impl<'a> fmt::Display for HexByteStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<HexByteStr<'a>> for String {
    fn from(hex_byte_string: HexByteStr) -> Self {
        hex_byte_string.0.to_string()
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
            (Some(first), Some(last)) => Some((HexChar::new(first).expect(ERR_UNREACHABLE),
                                               HexChar::new(last).expect(ERR_UNREACHABLE))),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct IntoIter {
    iter: OwnedChars,
}

impl<'a> IntoIterator for HexByteStr<'a> {
    type Item = HexCharPair;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter { iter: self.0.to_string().into_chars() }
    }
}

impl Iterator for IntoIter {
    type Item = HexCharPair;

    //TODO: DRY Iterator for Iter's & IntoIter's next() implementations (see above)
    fn next(&mut self) -> Option<HexCharPair> {
        match (self.iter.next(), self.iter.next()) {
            (Some(first), Some(last)) => Some((first.as_hex_char().expect(ERR_UNREACHABLE),
                                               last.as_hex_char().expect(ERR_UNREACHABLE))),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

impl<'a> IntoIterator for &'a HexByteStr<'a> {
    type Item = HexCharPair;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

impl<'a> From<ByteBuffer> for HexByteStr<'a> {
    fn from(byte_buffer: ByteBuffer) -> Self {
        HexByteStr::new("0123456789abcdef").ok().unwrap()
    }
}
