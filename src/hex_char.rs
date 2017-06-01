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
