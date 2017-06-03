pub mod consts;

use std;
pub use self::consts::*;
use hex_char::FromCharExtTrait;
use hex_byte_str::HexByteStr;

pub use fluent_validator::{Validator, FluentValidator, Error};

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)] mod unit_tests;

impl<'a, T: AsRef<str>> Validator<T> for HexByteStr<'a> {
    fn validate(value: T) -> Result<T> {
        Ok(value.as_ref())

            //value contains data?
            .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string())),
            })

            //value defines whole bytes (contains an even number of hex digits)?
            .and_then(|v| match v.len() % 2 == 0 {
                true => Ok(v),
                false => Err(Error::InvalidSize(VAL_ERR_INVALID_SIZE.to_string())),
            })

            //value contains only valid hexadecimal characters?
            .and_then(|v| match v.chars()
                                 .all(|c| c.is_hex_char()) {
                true => Ok(v),
                false => Err(Error::IllegalValue((VAL_ERR_ILLEGAL_HEX_DIGIT.to_string()))),
            })?;
        Ok(value)
    }
}
