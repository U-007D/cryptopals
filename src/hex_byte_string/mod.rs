mod consts;
use self::consts::msgs;
use super::fluent_validator::{FluentValidator, Error};
pub mod validator;

use std;
use std::fmt;
type Result<T> = std::result::Result<T, Error>;

#[cfg(test)] mod unit_tests;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HexByteString(String);

impl HexByteString {
    pub fn new<T: AsRef<str>>(value: T) -> Result<HexByteString> {
        Ok(HexByteString(value.as_ref().validate::<HexByteString>()?.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for HexByteString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<HexByteString> for String {
    fn from(hex_byte_string: HexByteString) -> Self {
        hex_byte_string.0
    }
}
