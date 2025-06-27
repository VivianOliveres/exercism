pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None
    }

    let mut count = 0;
    while n != 1 {
        match n % 2 {
            0 => n /= 2,
            _ => n = n * 3 + 1,
        }
        count += 1;
    }

    Some(count)
}

#[test]
fn zero_steps_for_one() {
    let output = collatz(1);
    let expected = Some(0);
    assert_eq!(output, expected);
}

#[test]
fn divide_if_even() {
    let output = collatz(16);
    let expected = Some(4);
    assert_eq!(output, expected);
}

#[test]
fn even_and_odd_steps() {
    let output = collatz(12);
    let expected = Some(9);
    assert_eq!(output, expected);
}

#[test]
fn large_number_of_even_and_odd_steps() {
    let output = collatz(1_000_000);
    let expected = Some(152);
    assert_eq!(output, expected);
}

#[test]
fn zero_is_an_error() {
    let output = collatz(0);
    let expected = None;
    assert_eq!(output, expected);
}
