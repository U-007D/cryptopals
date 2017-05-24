use super::*;

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)] mod unit_tests;

#[derive(Debug, PartialEq, Eq)]
pub struct HexByteStr<'a> {
    value: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ByteBuffer;

impl<'a> HexByteStr<'a> {
    pub fn new(init_value: &str) -> Result<HexByteStr> {
        Ok(HexByteStr { value: init_value.validate::<HexByteStr>()? })
    }

    pub fn as_hex_byte_str(&self) -> &'a str {
        self.value
    }

    pub fn as_byte_buffer(&self) -> ByteBuffer {
        ByteBuffer
    }
}

impl<'a> Validator<&'a str> for HexByteStr<'a> {
    fn validate(v: &'a str) -> Result<&'a str> where Self: Sized {
        match None
            .or_else(
                || match !v.is_empty() {
                    true => None,
                    false => Some(fluent_validator::Error::EmptyValue(String::new())),
                })
            .or_else(
                || match v.chars().all(|c| match c {
                                                '0'...'9' |
                                                'a'...'f' |
                                                'A'...'F' => true,
                                                _ => false,
                }) {
                    true => None,
                    false => Some(fluent_validator::Error::IllegalValue(String::new())),
                })
            .or_else(
                || match v.chars().count() % 2 == 0 {
                    true => None,
                    false => Some(fluent_validator::Error::InvalidSize(String::new())),
                }) {
            None => Ok(v),
            Some(e) => Err(e),
        }
    }
}
