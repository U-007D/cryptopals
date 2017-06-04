use consts::*;
use hex_byte_string;

/// Semantic typing for values as hexadecimal digits.  Ensures types intended to contain hexadecimal values are kept
/// both distinct and distinguishable from general values.  Allows APIs to enforcably express expectations, and permits
/// use of these values without having to validate before each use.  Note: all possible construction methods go through
/// validation to ensure that all instances of the type are guaranteed to be valid.
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

/// Extension Trait for ensuring correct range on values and/or converting values to hexadecimal characters
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
        hex_byte_string::HEX_DIGITS.chars().nth(*self as  usize)
    }

//TODO: Determine if using From is preferable to as_hex_char()
//impl From<HexValue> for char {
//    fn from(hex_val: HexValue) -> Self {
//      consts::HEX_DIGITS.chars().nth(hex_val)
//    }
}
