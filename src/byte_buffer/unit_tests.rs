use super::*;
use self::validator::*;

#[test]
fn from_hex_str_rejects_empty_input() {
    let input = "";
    let expected_result = Error::EmptyValue(VAL_ERR_EMPTY_VALUE.to_string());

    assert!(ByteBuffer::from_hex_str(input).err().unwrap() == expected_result);
}
