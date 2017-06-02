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
