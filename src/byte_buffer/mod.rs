mod consts;

use hex_char_byte_string::HexCharByteString;
use hex_char::HexChar;

#[cfg(test)] mod unit_tests;

#[derive(Debug)]
pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    pub fn as_byte_vec(&self) -> Vec<u8> {
        self.0.clone()
    }
}

impl From<HexCharByteString> for ByteBuffer {
    fn from(hex_char_byte_string: HexCharByteString) -> Self {
        ByteBuffer(hex_char_byte_string.into_iter()
                                  .map(|hcp| u8::from(hcp.0) << consts::BITS_PER_HEX_CHAR |
                                            u8::from(hcp.1))
                                  .collect())
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
