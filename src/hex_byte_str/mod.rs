use super::*;

type Result<T> = std::result::Result<T, Error>;

#[cfg(test)] mod unit_tests;

#[derive(Debug, PartialEq, Eq)]
pub struct HexByteStrValidator<'a> {
    value: &'a str,
}

impl<'a> Validator<&'a str> for HexByteStrValidator<'a> {
    fn validate(v: &'a str) -> Result<Self> where Self: Sized {
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
            None => Ok(HexByteStrValidator { value: v }),
            Some(e) => Err(e),
        }
    }
}
