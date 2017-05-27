use super::*;
use super::hex_byte_string::HexByteString;
use std::fmt;

type Result<T> = std::result::Result<T, fluent_validator::Error>;

#[cfg(test)] mod unit_tests;
mod consts;
use hex_byte_string::HexByteChars;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ByteBuffer {
    value: Vec<u8>,
}

impl<'a> ByteBuffer {
    pub fn from_hex_byte_string(init_value: HexByteString) -> ByteBuffer {
        let mut result = Vec::<u8>::with_capacity(init_value.len() / 2);
        for hex_byte_chars in &init_value {
            result.push(ByteBuffer::hex_byte_chars_as_value(hex_byte_chars));
        }
        ByteBuffer { value: result }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.value
    }

    pub fn as_hex_byte_string(&self) -> String {
        let mut result = String::with_capacity(self.value.len() * 2);
        for byte in &self.value {
            result.push(ByteBuffer::value_into_hex_char(byte >> consts::BITS_PER_NIBBLE));
            result.push(ByteBuffer::value_into_hex_char(byte & consts::HIGH_NIBBLE_MASK));
        }
        result
    }

    pub fn iter(&self) -> ByteBufferIter {
        ByteBufferIter { value: &self.value, pos: 0 }
    }

    fn value_into_hex_char(value: u8) -> char {
        match value {
            0...9 => (b'0' + value) as char,
            10...15 => (b'a' + value - consts::HEX_A_VALUE) as char,
            _ => unreachable!(),
        }
    }

    fn hex_byte_chars_as_value(hex_byte_chars: HexByteChars) -> u8 {
        ByteBuffer::hex_char_into_value(hex_byte_chars[consts::HEX_BYTE_CHARS_MSN]) << consts::BITS_PER_NIBBLE |
                ByteBuffer::hex_char_into_value(hex_byte_chars[consts::HEX_BYTE_CHARS_LSN])
    }

    fn hex_char_into_value(ch: char) -> u8 {
        match ch as u8 {
            v @ consts::UTF8_0_VALUE...consts::UTF8_9_VALUE => v - consts::UTF8_0_VALUE,
            v @ consts::UTF8_UPPER_A_VALUE...consts::UTF8_UPPER_F_VALUE =>
                    v - consts::UTF8_UPPER_A_VALUE + consts::HEX_A_VALUE,
            v @ consts::UTF8_LOWER_A_VALUE...consts::UTF8_LOWER_F_VALUE =>
                    v - consts::UTF8_LOWER_A_VALUE + consts::HEX_A_VALUE,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct ByteBufferIter<'a> {
    value: &'a [u8],
    pos: usize,
}

impl<'a> Iterator for ByteBufferIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        None
    }
}
