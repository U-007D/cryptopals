use std::fmt;
use std::ops;

mod consts;

use hex_byte_string;
use hex_byte_string::HexByteString;

#[cfg(test)] mod unit_tests;

#[derive(Debug)]
pub struct ByteBuffer(Vec<u8>);

/// ByteBuffer is designed to be constructed using ByteBuffer::from_*() methods.  This is because a ByteBuffer can be
/// created from any number of sources (eg. HexByteString or a Base64String), and no particular format is preferred.
/// There is also no use-case for an empty ByteBuffer at this time, so there is no need for new().
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

/// Gives ByteBuffer a to_string() method (note to_string() above is private).  (Just thought I'd try it out)
impl From<ByteBuffer> for String {
    fn from(byte_buffer: ByteBuffer) -> Self { ByteBuffer::to_string(&byte_buffer) }
}

/// Gives Into<ByteBuffer> methods (not used in this project at this point)
impl From<HexByteString> for ByteBuffer {
    fn from(hex_byte_string: HexByteString) -> Self {
        ByteBuffer(hex_byte_string.iter()
                                       .map(|hex_char_pair| u8::from(hex_char_pair.0) << hex_byte_string::BITS_PER_HEX_DIGIT |
                                                            u8::from(hex_char_pair.1))
                                       .collect())
    }
}

/// Gives ByteBuffer all of Vec<u8>'s Iter and IntoIter methods
impl ops::Deref for ByteBuffer {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
