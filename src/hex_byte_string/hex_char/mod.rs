use hex_byte_string;

/// Semantic type for a hexadecimal character.  Not possible to encode a non-hexadecimal character in this type, so
/// consumers of this type are guaranteed valid hexadecimal input.
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

/// Determines whether the given input is a valid hexadecimal character.
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

/// Permits conversion of hex character to a value.
impl From<HexChar> for u8 {
    fn from(hex_char: HexChar) -> Self {
        match hex_char.as_char() {
            ch @ '0'...'9' => ch as u8 - '0' as u8,
            ch @ 'A'...'F' => ch as u8 - 'A' as u8 + hex_byte_string::HEX_A_VALUE,
            ch @ 'a'...'f' => ch as u8 - 'a' as u8 + hex_byte_string::HEX_A_VALUE,
            _ => unreachable!(),
        }
    }
}
