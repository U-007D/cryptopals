use super::*;

#[test]
fn new_accepts_valid_input() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    assert!(if let Some(HexByteString(_)) = HexByteString::new(input).ok() { true } else { false });
}

#[test]
fn new_rejects_empty_input() {
    let input = "";
    assert!(if let Some(Error::EmptyValue(_)) = HexByteString::new(input).err() { true } else { false });
}

#[test]
fn new_rejects_odd_length_hex_input() {
    let input = "f";
    assert!(if let Some(Error::InvalidSize(_)) = HexByteString::new(input).err() { true } else { false });
}

#[test]
fn new_rejects_illegal_hex_digit() {
    let input = "abcdefgh";
    assert!(if let Some(Error::IllegalValue(_)) = HexByteString::new(input).err() { true } else { false });
}


#[test]
fn implements_as_str() {
    let input = "c0debeef";
    let expected_result = input.clone();

    assert!(HexByteString::new(input).unwrap().as_str() == expected_result);
}
