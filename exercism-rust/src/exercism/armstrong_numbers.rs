// Version FP
// pub fn is_armstrong_number(num: u32) -> bool {
//     let s = num.to_string();
//     let len = s.len() as u32;
//
//     s.chars()
//         .map(|c| c.to_digit(10).unwrap().pow(len))
//         .sum::<u32>() == num
// }

pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let number_digits: u32 = count_digits(num);
    let mut sum = 0;
    let mut rest = num;
    while rest > 0 {
        let current_digit = rest % 10;
        sum += current_digit.pow(number_digits);
        rest /= 10
    }

    sum == num
}

pub fn count_digits(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }

    let mut count = 0;
    let mut rest = num;
    while rest > 0 {
        count += 1;
        rest /= 10;
    }
    count
}

#[test]
fn zero_is_an_armstrong_number() {
    assert!(is_armstrong_number(0))
}

#[test]
fn single_digit_numbers_are_armstrong_numbers() {
    assert!(is_armstrong_number(5))
}

#[test]
fn there_are_no_two_digit_armstrong_numbers() {
    assert!(!is_armstrong_number(10))
}

#[test]
fn three_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(153))
}

#[test]
fn three_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(100))
}

#[test]
fn four_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_474))
}

#[test]
fn four_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_475))
}

#[test]
fn seven_digit_number_that_is_an_armstrong_number() {
    assert!(is_armstrong_number(9_926_315))
}

#[test]
fn seven_digit_number_that_is_not_an_armstrong_number() {
    assert!(!is_armstrong_number(9_926_314))
}
