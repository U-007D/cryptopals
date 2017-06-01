pub mod consts;

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
            hc @ 'A'...'F' => hc as u8 - 'A' as u8 + consts::HEX_A_VALUE,
            hc @ 'a'...'f' => hc as u8 - 'a' as u8 + consts::HEX_A_VALUE,
            _ => unreachable!(),
        }
    }
}
