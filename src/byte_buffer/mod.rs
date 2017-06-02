mod validator;

use std;
use std::convert::TryFrom;
use std::str::Chars;
use self::validator::{FluentValidator, HexStr};
use hex_char;
use hex_char::HexChar;
use super::consts::ERR_UNREACHABLE;

#[cfg(test)] mod unit_tests;

type Result<T> = std::result::Result<T, validator::Error>;
type HexCharPair = (HexChar, HexChar);

pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    fn from_hex_str(init_value: &str) -> Result<ByteBuffer> {
        ByteBuffer::try_from(init_value)
    }
}

impl<'a> TryFrom<&'a str> for ByteBuffer {
    type Error = validator::Error;

    fn try_from(init_value: &str) -> Result<ByteBuffer> {
        Ok(ByteBuffer(init_value.validate::<HexStr>()?
                                .hex_char_iter()
                                .map(|hex_char_pair| u8::from(hex_char_pair.0) << hex_char::BITS_PER_HEX_CHAR |
                                                     u8::from(hex_char_pair.1))
                                .collect()
        ))
    }
}

trait BaseNExtTrait {
    fn hex_char_iter(&self) -> HexCharIter;
}

impl<T: AsRef<str>> BaseNExtTrait for T {
    fn hex_char_iter(&self) -> HexCharIter {
        HexCharIter{ iter: self.as_ref().chars() }
    }
}

#[derive(Debug)]
pub struct HexCharIter<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for HexCharIter<'a> {
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
