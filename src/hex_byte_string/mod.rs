#[cfg(test)] mod unit_tests;

use std;
use super::Error;

pub struct HexByteString(String);

type Result<T> = std::result::Result<T, Error>;

impl HexByteString {
    pub fn new<T: Into<String>>(init_value: T) -> Result<HexByteString> {
        Ok(HexByteString(init_value.into()))
    }
}
