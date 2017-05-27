use super::*;

#[test]
fn empty_test() {}

#[test]
fn byte_buffer_from_hex_byte_str_succeeds() {
    let input = HexByteString::new(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()).unwrap();
    let byte_buffer = ByteBuffer::from_hex_byte_string(input.clone());
    let expected_result = input.as_string();

    assert_eq!(byte_buffer.as_hex_byte_string(), expected_result);
}

#[test]
fn byte_buffer_as_byte_buffer_succeeds() {
    let input = HexByteString::new(
            "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string()).unwrap();
    let byte_buffer = ByteBuffer::from_hex_byte_string(input);
    #[allow(trivial_casts)] //Rust only coerces automatically from &[T; n] => &[T] when n <= 32
    let expected_result = &[0x49u8, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x62,
             0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e,
             0x6f, 0x75, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d] as &[u8];

    assert!(byte_buffer.as_slice() == expected_result);
}
