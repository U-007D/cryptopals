use super::*;

#[test]
fn empty_test() {}

#[test]
fn hex_byte_str_as_hex_byte_str_succeeds() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_byte_str = HexByteStr::new(input).ok().unwrap();
    let expected_result = input.clone();

    assert_eq!(hex_byte_str.as_hex_byte_str(), expected_result);
}

#[test]
fn hex_byte_str_as_byte_buffer_succeeds() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let hex_byte_str = HexByteStr::new(input).ok().unwrap();
    let expected_result = ByteBuffer;

    assert_eq!(hex_byte_str.as_byte_buffer(), expected_result);
}
