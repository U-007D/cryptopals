use super::*;

use fluent_validator::Validator;

type Result<T> = std::result::Result<T, Error>;

impl<T: AsRef<str>> Validator<T> for HexByteString {
    fn validate(value: T) -> Result<T> {
        Ok(value.as_ref())
            .and_then(|v| match !v.is_empty() {
                true => Ok(v),
                false => Err(Error::EmptyValue(msgs::VALDN_ERR_EMPTY_VALUE.to_string())),
            })
            .and_then(|v| match v.len() % 2 == 0 {
                true => Ok(v),
                false => Err(Error::InvalidSize(msgs::VALDN_ERR_INVALID_SIZE.to_string())),
            })?;
        Ok(value)
    }
}
