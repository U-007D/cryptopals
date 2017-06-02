use super::*;
use self::validator::*;

#[test]
fn from_hex_str_rejects_empty_input() {
    let input = "";
    let expected_result = Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string());

    assert!(ByteBuffer::from_hex_str(input).err().unwrap() == expected_result);
}

#[test]
fn from_hex_str_accepts_valid_input() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert!(if let Some(ByteBuffer(_)) = ByteBuffer::from_hex_str(input).ok() { true } else { false });
}

#[test]
fn from_hex_str_rejects_odd_length_hex_input() {
    let input = "f";
    assert!(if let Some(Error::InvalidSize(_)) = ByteBuffer::from_hex_str(input).err() { true } else { false });
}

#[test]
fn from_hex_str_rejects_illegal_hex_digit() {
    let input = "abcdefgh";
    assert!(if let Some(Error::IllegalValue(_)) = ByteBuffer::from_hex_str(input).err() { true } else { false });
}

#[test]
fn implements_from_trait_for_string() {
    let input = "deadbeef";
    let expected_result = "222, 173, 190, 239, ".to_string();

    assert!(String::from(ByteBuffer::from_hex_str(input).unwrap()) == expected_result);
}

#[test]
fn implements_to_string() {
    let input = "deadc0de";
    let expected_result = "222, 173, 192, 222, ".to_string();

    assert!(ByteBuffer::from_hex_str(input).unwrap().to_string() == expected_result);
}
