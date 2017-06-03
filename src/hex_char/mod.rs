mod consts;

use consts::ERR_UNREACHABLE;
pub use self::consts::*;
use std::str::Chars;
use owned_chars::{OwnedChars, OwnedCharsExt};

type HexCharPair = (HexChar, HexChar);

#[cfg(test)] mod unit_tests;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HexChar {
    char: char,
}

impl HexChar {
    pub fn new(ch: char) -> Option<HexChar> {
        match ch.is_hex_char() {
            true => Some(HexChar { char: ch }),
            false => None,
        }
    }

    pub fn as_char(&self) -> char {
        self.char
    }
}

pub trait IsHexChar {
    fn is_hex_char(&self) -> bool;
}

impl IsHexChar for char {
    fn is_hex_char(&self) -> bool {
        match *self {
            '0' ... '9' |
            'A' ... 'F' |
            'a' ... 'f' => true,
            _ => false,
        }
    }
}

impl From<HexChar> for u8 {
    fn from(hex_char: HexChar) -> Self {
        match hex_char.as_char() {
            hc @ '0'...'9' => hc as u8 - '0' as u8,
            hc @ 'A'...'F' => hc as u8 - 'A' as u8 + HEX_A_VALUE,
            hc @ 'a'...'f' => hc as u8 - 'a' as u8 + HEX_A_VALUE,
            _ => unreachable!(),
        }
    }
}

pub trait HexCharExtTrait {
    fn hex_char_iter(&self) -> HexCharIter;
}

pub trait OwnedHexCharExtTrait {
    fn into_hex_char_iter(self) -> OwnedHexCharIter;
}

impl<T: AsRef<str>> HexCharExtTrait for T {
    fn hex_char_iter(&self) -> HexCharIter {
        HexCharIter { iter: self.as_ref().chars() }
    }
}

impl<T: Into<String>> OwnedHexCharExtTrait for T {
    fn into_hex_char_iter(self) -> OwnedHexCharIter {
        OwnedHexCharIter { iter: self.into().into_chars()}
    }
}

#[derive(Debug)]
pub struct HexCharIter<'a> {
    iter: Chars<'a>,
}

#[derive(Debug)]
pub struct OwnedHexCharIter {
    iter: OwnedChars,
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

impl Iterator for OwnedHexCharIter {
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
