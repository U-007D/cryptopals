use super::*;

type Result<T> = std::result::Result<T, fluent_validator::Error>;
pub type HexByteChars = [char; consts::HEX_CHARS_PER_BYTE];

#[cfg(test)] mod unit_tests;
mod consts;
use self::consts::*;
mod validator;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HexByteString {
    value: String,
}

impl HexByteString {
    pub fn new(init_value: String) -> Result<HexByteString> {
        Ok(HexByteString { value: init_value.validate::<HexByteString>()? } )
    }

    pub fn as_str(&self) -> &str { self.value.as_str() }
    pub fn as_string(&self) -> String { self.value.clone() }
    pub fn iter(&self) -> HexByteStringIter {
        HexByteStringIter { value: &self.value, str_iter: (&self.value).chars(), }
    }
    pub fn len(&self) -> usize { self.value.len() }
}

#[derive(Debug)]
pub struct HexByteStringIter<'a> {
    value: &'a str,
    str_iter: Chars<'a>
}

impl<'a> IntoIterator for &'a HexByteString {
    type Item = [char; consts::HEX_CHARS_PER_BYTE];
    type IntoIter = HexByteStringIter<'a>;

    fn into_iter(self) -> HexByteStringIter<'a> {
        self.iter()
    }
}

impl<'a> Iterator for HexByteStringIter<'a> {
    type Item = HexByteChars;

    fn next(&mut self) -> Option<HexByteChars> {
        match (self.str_iter.next(), self.str_iter.next()) {
            (None, None) => None,
            (Some(first), Some(last)) => Some([first, last]),
            _ => unreachable!(),  //Invalid state: HexByteStr must always contain even # of ASCII chars
        }
    }
}
