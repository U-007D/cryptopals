mod consts;

pub use self::consts::*;
use std::str::Chars;
use owned_chars::{OwnedChars, OwnedCharsExt};

pub type HexCharPair = (HexChar, HexChar);

#[cfg(test)] mod unit_tests;

pub struct HexChar(char);

impl HexChar {
    pub fn new(ch: char) -> Option<HexChar> {
        match ch.is_hex_char() {
            true => Some(HexChar(ch)),
            false => None,
        }
    }
}

pub trait FromCharExtTrait {
    fn is_hex_char(&self) -> bool;
    fn as_hex_char(&self) -> Option<HexChar>;
}

impl FromCharExtTrait for char {
    fn is_hex_char(&self) -> bool {
        match *self {
            '0' ... '9' |
            'A' ... 'F' |
            'a' ... 'f' => true,
            _ => false,
        }
    }

    fn as_hex_char(&self) -> Option<HexChar> {
        HexChar::new(*self)
    }
}

pub trait HexValue {
    fn as_hex_value(&self) -> u8;
}

impl HexValue for HexChar {
    fn as_hex_value(&self) -> u8 {
        match self.0 {
            ch @ '0'...'9' => ch as u8 - '0' as u8,
            ch @ 'A'...'F' => ch as u8 - 'A' as u8 + HEX_A_VALUE,
            ch @ 'a'...'f' => ch as u8 - 'a' as u8 + HEX_A_VALUE,
            _ => unreachable!(),
        }
    }
}
