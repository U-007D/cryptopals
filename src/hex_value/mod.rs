use consts::*;
//TODO: Refactor hex_char and hex_value as peer submodules sharing same const folders
use hex_char::consts;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct HexValue(u8);

impl HexValue {
    pub fn new(val: u8) -> Option<HexValue> {
        match val.is_hex_value() {
            true => Some(HexValue(val)),
            false => None,
        }
    }

    pub fn as_value(&self) -> u8 {
        self.0
    }

    pub fn as_hex_char(&self) -> char {
        self.0.as_hex_char().expect(ERR_UNREACHABLE)
    }
}

pub trait HexValueExt {
    fn is_hex_value(&self) -> bool;
    fn as_hex_char(&self) -> Option<char>;
}

impl HexValueExt for u8 {
    fn is_hex_value(&self) -> bool {
        match *self {
            0 ... 15 => true,
            _ => false,
        }
    }

    fn as_hex_char(&self) -> Option<char> {
        consts::HEX_DIGITS.chars().nth(*self as  usize)
    }

//TODO: Determine if using From is preferable to as_hex_char()
//impl From<HexValue> for char {
//    fn from(hex_val: HexValue) -> Self {
//      consts::HEX_DIGITS.chars().nth(hex_val)
//    }
}
