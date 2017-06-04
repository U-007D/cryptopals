use super::*;
use fluent_validator::FluentValidator;

#[test]
fn accepts_valid_str() {
    let input = "ff";
    let expected_result = Ok(input.clone());

    assert!(input.validate::<HexByteString>() == expected_result);
}

#[test]
fn accepts_valid_string() {
    let input = "ee".to_string();
    let expected_result = Ok(input.clone());

    assert!(input.validate::<HexByteString>() == expected_result);
}

#[test]
fn rejects_empty_str() {
    let input = "";
    let expected_result = Err(Error::EmptyValue(msgs::VAL_ERR_EMPTY_VALUE.to_string()));

    assert!(input.validate::<HexByteString>() == expected_result);
}

#[test]
fn rejects_odd_length_str() {
    let input = "6";
    let expected_result = Err(Error::InvalidSize(msgs::VAL_ERR_INVALID_SIZE.to_string()));

    assert!(input.validate::<HexByteString>() == expected_result);
}

#[test]
fn rejects_invalid_hex_char_in_str() {
    let input = "12345x";
    let expected_result = Err(Error::IllegalValue(msgs::VAL_ERR_ILLEGAL_HEX_DIGIT.to_string()));

    assert!(input.validate::<HexByteString>() == expected_result);
}
