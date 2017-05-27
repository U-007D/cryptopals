use super::*;

#[cfg(test)] mod unit_tests;


impl<'a> Validator<&'a str> for HexByteString {
    fn validate<'b>(v: &'b str) -> Result<&'b str> {
        v.to_string().validate::<HexByteString>()?;
        Ok(v)
    }
}

impl Validator<String> for HexByteString {
    fn validate(v: String) -> Result<String> {
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
