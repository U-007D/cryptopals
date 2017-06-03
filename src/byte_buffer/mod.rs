mod validator;

use std;
use std::convert::TryFrom;
use std::fmt;
use self::validator::{FluentValidator, HexStr};
use hex_char;
use hex_char::HexCharExtTrait;

#[cfg(test)] mod unit_tests;

type Result<T> = std::result::Result<T, validator::Error>;

#[derive(Debug)]
pub struct ByteBuffer(Vec<u8>);

impl ByteBuffer {
    fn from_hex_str(init_value: &str) -> Result<ByteBuffer> {
        ByteBuffer::try_from(init_value)
    }

    fn to_string(&self) -> String {
        self.0.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", ")
    }
}

impl<'a> TryFrom<&'a str> for ByteBuffer {
    type Error = validator::Error;

    fn try_from(init_value: &str) -> Result<ByteBuffer> {
        Ok(ByteBuffer(init_value.validate::<HexStr>()?
                                .hex_char_iter()
                                .map(|hex_char_pair| u8::from(hex_char_pair.0) << hex_char::BITS_PER_HEX_CHAR |
                                                     u8::from(hex_char_pair.1))
                                .collect()
        ))
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
