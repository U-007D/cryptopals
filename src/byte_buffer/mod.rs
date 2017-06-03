use std;
use std::fmt;
use hex_char;
use hex_char::HexValue;
use hex_byte_str;
use hex_byte_str::{HexByteStr, Iter};

#[cfg(test)] mod unit_tests;

type Result<T> = std::result::Result<T, hex_byte_str::validator::Error>;

#[derive(Debug)]
pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    pub fn from_hex_str<T: AsRef<str>>(init_value: T) -> Result<ByteBuffer> {
        Ok(ByteBuffer(HexByteStr::new(init_value.as_ref())?
                          .into_iter()
                          .map(|hex_char_pair| hex_char_pair.0.as_hex_value() << hex_char::BITS_PER_HEX_CHAR |
                                               hex_char_pair.1.as_hex_value())
                          .collect()
        ))
    }

//    pub fn to_hex_string(&self) -> String {
//
//    }

    fn to_string(&self) -> String {
        self.0.iter().fold(String::new(), |acc, &byte| acc + &byte.to_string() + ", ")
    }
}

impl fmt::Display for ByteBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.to_string())
    }
}

impl From<ByteBuffer> for String {
    fn from(byte_buffer: ByteBuffer) -> Self { byte_buffer.to_string() }
}
