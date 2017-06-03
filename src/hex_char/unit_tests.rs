use super::*;

#[test]
fn hex_char_iter_yields_char_pairs() {
    let data = "1234";
    let mut input = data.hex_char_iter();
    let expected_results = [Some((HexChar::new('1').unwrap(), HexChar::new('2').unwrap())),
        Some((HexChar::new('3').unwrap(), HexChar::new('4').unwrap())), None];

    assert!(input.next() == expected_results[0]);
    assert!(input.next() == expected_results[1]);
    assert!(input.next() == expected_results[2]);
}

#[test]
fn into_hex_char_iter_yields_char_pairs() {
    let mut input = "1234".into_hex_char_iter();
    let expected_results = [Some((HexChar::new('1').unwrap(), HexChar::new('2').unwrap())),
        Some((HexChar::new('3').unwrap(), HexChar::new('4').unwrap())), None];

    assert!(input.next() == expected_results[0]);
    assert!(input.next() == expected_results[1]);
    assert!(input.next() == expected_results[2]);
}
