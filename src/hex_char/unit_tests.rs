use super::*;

#[test]
fn hex_char_iter_yields_char_pairs() {
    let mut input = HexChar::new("1234").iter();
    let expected_results = [Some(('1', '2')), Some(('3', '4')), None];

    assert!(input.next() == expected_results[0]);
    assert!(input.next() == expected_results[1]);
    assert!(input.next() == expected_results[2]);
}

#[test]
fn into_hex_char_iter_yields_char_pairs() {
    let mut input = HexChar::new("1234").into_iter();
    let expected_results = [Some(('1', '2')), Some(('3', '4')), None];

    assert!(input.next() == expected_results[0]);
    assert!(input.next() == expected_results[1]);
    assert!(input.next() == expected_results[2]);
}
