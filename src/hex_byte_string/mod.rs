#[cfg(test)] mod unit_tests;

mod consts;
use self::consts::msgs;
use super::fluent_validator::Error;

use std;

type Result<T> = std::result::Result<T, Error>;

pub struct HexByteString(String);

impl HexByteString {
    pub fn new<T: Into<String>>(init_value: T) -> Result<HexByteString> {
        let init_value = init_value.into();
        match init_value == String::new() {
            true => Err(Error::EmptyValue(msgs::ERR_EMPTY_VALUE.to_string())),
            false => Ok(HexByteString(init_value)),
        }
    }
}
