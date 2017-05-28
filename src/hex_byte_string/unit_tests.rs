use super::HexByteString;

#[test]
fn new_accepts_valid_input() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected_result = true;
    assert!(HexByteString::new(input).is_ok() == expected_result);
}
