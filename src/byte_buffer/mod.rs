mod consts;

use std::fmt;
use hex_char_byte_string::HexCharByteString;
use hex_char::HexChar;
use hex_char::consts::BITS_PER_HEX_CHAR;

#[cfg(test)] mod unit_tests;

#[derive(Debug)]
pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    pub fn as_byte_vec(&self) -> Vec<u8> {
        self.0.clone()
    }

    fn to_string(&self) -> String {
        self.0.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", ")
    }
}
impl fmt::Display for ByteBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.to_string())
    }
}

impl From<ByteBuffer> for String {
    fn from(byte_buffer: ByteBuffer) -> Self { ByteBuffer::to_string(&byte_buffer) }
}

impl From<HexCharByteString> for ByteBuffer {
    fn from(hex_char_byte_string: HexCharByteString) -> Self {
        ByteBuffer(hex_char_byte_string.iter()
                                       .map(|hex_char_pair| u8::from(hex_char_pair.0) << BITS_PER_HEX_CHAR |
                                                            u8::from(hex_char_pair.1))
                                       .collect())
    }
}
