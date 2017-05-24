use super::*;

#[test]
fn hex_byte_str_validator_handles_empty_input() {
    let input = "";
    let expected_result = Some(fluent_validator::Error::EmptyValue(String::new()));

    assert_eq!(input.validate::<HexByteStr>().err(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_valid_hex_input() {
    let input = "0123456789abcdefABCDEF";
    let expected_result = Some(input.clone());

    assert_eq!(input.validate::<HexByteStr>().ok(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_invalid_hex_input() {
    let input = "0123456789xxabcdefABCDEF";
    let expected_result = Some(fluent_validator::Error::IllegalValue(String::new()));

    assert_eq!(input.validate::<HexByteStr>().err(), expected_result);
}

#[test]
fn hex_byte_str_validator_handles_odd_length_hex_input() {
    let input = "0123456789abcdefABCDE";
    let expected_result = Some(fluent_validator::Error::InvalidSize(String::new()));

    assert_eq!(input.validate::<HexByteStr>().err(), expected_result);
}
