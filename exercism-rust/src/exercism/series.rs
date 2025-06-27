pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }

    let mut results: Vec<String> = Vec::with_capacity(digits.len() - len + 1);
    for i in 0..=(digits.len() - len) {
        let substring: &str = &digits[i..(i + len)];
        results.push(substring.to_owned()); // to_owned is a little bit faster than to_string (for &str)
    }
    results

    // FP and idiomatic version but less efficient
    // digits
    //     .as_bytes()
    //     .windows(len)
    //     .map(|window| std::str::from_utf8(window).unwrap().to_string())
    //     .collect()
}

#[test]
fn slices_of_one_from_one() {
    let input = "1";
    let length = 1;
    let output = series(input, length);
    let expected = &["1"];
    assert_eq!(output, expected);
}

#[test]
fn slices_of_one_from_two() {
    let input = "12";
    let length = 1;
    let output = series(input, length);
    let expected = &["1", "2"];
    assert_eq!(output, expected);
}

#[test]
fn slices_of_two() {
    let input = "35";
    let length = 2;
    let output = series(input, length);
    let expected = &["35"];
    assert_eq!(output, expected);
}

#[test]
fn slices_of_two_overlap() {
    let input = "9142";
    let length = 2;
    let output = series(input, length);
    let expected = &["91", "14", "42"];
    assert_eq!(output, expected);
}

#[test]
fn slices_can_include_duplicates() {
    let input = "777777";
    let length = 3;
    let output = series(input, length);
    let expected = &["777", "777", "777", "777"];
    assert_eq!(output, expected);
}

#[test]
fn slices_of_a_long_series() {
    let input = "918493904243";
    let length = 5;
    let output = series(input, length);
    let expected = &[
        "91849", "18493", "84939", "49390", "93904", "39042", "90424", "04243",
    ];
    assert_eq!(output, expected);
}

#[test]
fn slice_length_is_too_large() {
    let input = "12345";
    let length = 6;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}

#[test]
fn slice_length_is_way_too_large() {
    let input = "12345";
    let length = 42;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}

#[test]
fn empty_series_is_invalid() {
    let input = "";
    let length = 1;
    let output = series(input, length);
    let expected: &[&str] = &[];
    assert_eq!(output, expected);
}
