use std::fmt::Write;

pub const THIRD_LINE: &str = "And if one green bottle should accidentally fall,\n";

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut current_bottle = start_bottles;
    let end_bottle = start_bottles - take_down;
    let mut result = String::new();

    while current_bottle > end_bottle {
        let (left, right) = get_name(current_bottle);
        let first_plural = get_bottle_word(current_bottle);
        let second_plural = get_bottle_word(current_bottle - 1);

        writeln!(result, "{left} green {first_plural} hanging on the wall,").unwrap();
        writeln!(result, "{left} green {first_plural} hanging on the wall,").unwrap();
        result.push_str(THIRD_LINE);
        write!(
            result,
            "There'll be {right} green {second_plural} hanging on the wall."
        )
        .unwrap();

        current_bottle -= 1;
        if current_bottle > end_bottle {
            result.push_str("\n\n");
        }
    }

    result
}

// Use a `&'static str` instead of `String` in order to return constant values without reallocation.
fn get_bottle_word(value: u32) -> &'static str {
    if value == 0 {
        "bottles"
    } else if value > 1 {
        "bottles"
    } else {
        "bottle"
    }
}

fn get_name(value: u32) -> (&'static str, &'static str) {
    match value {
        10 => ("Ten", "nine"),
        9 => ("Nine", "eight"),
        8 => ("Eight", "seven"),
        7 => ("Seven", "six"),
        6 => ("Six", "five"),
        5 => ("Five", "four"),
        4 => ("Four", "three"),
        3 => ("Three", "two"),
        2 => ("Two", "one"),
        1 => ("One", "no"),
        _ => panic!("{value} is not managed"),
    }
}

#[test]
fn first_generic_verse() {
    assert_eq!(
        recite(10, 1).trim(),
        concat!(
            "Ten green bottles hanging on the wall,\n",
            "Ten green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be nine green bottles hanging on the wall.",
        )
    );
}

#[test]
fn last_generic_verse() {
    assert_eq!(
        recite(3, 1).trim(),
        concat!(
            "Three green bottles hanging on the wall,\n",
            "Three green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be two green bottles hanging on the wall.",
        )
    );
}

#[test]
fn verse_with_2_bottles() {
    assert_eq!(
        recite(2, 1).trim(),
        concat!(
            "Two green bottles hanging on the wall,\n",
            "Two green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be one green bottle hanging on the wall.",
        )
    );
}

#[test]
fn verse_with_1_bottle() {
    assert_eq!(
        recite(1, 1).trim(),
        concat!(
            "One green bottle hanging on the wall,\n",
            "One green bottle hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be no green bottles hanging on the wall.",
        )
    );
}

#[test]
fn first_two_verses() {
    assert_eq!(
        recite(10, 2).trim(),
        concat!(
            "Ten green bottles hanging on the wall,\n",
            "Ten green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be nine green bottles hanging on the wall.\n",
            "\n",
            "Nine green bottles hanging on the wall,\n",
            "Nine green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be eight green bottles hanging on the wall.",
        )
    );
}

#[test]
fn last_three_verses() {
    assert_eq!(
        recite(3, 3).trim(),
        concat!(
            "Three green bottles hanging on the wall,\n",
            "Three green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be two green bottles hanging on the wall.\n",
            "\n",
            "Two green bottles hanging on the wall,\n",
            "Two green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be one green bottle hanging on the wall.\n",
            "\n",
            "One green bottle hanging on the wall,\n",
            "One green bottle hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be no green bottles hanging on the wall.",
        )
    );
}

#[test]
fn all_verses() {
    assert_eq!(
        recite(10, 10).trim(),
        concat!(
            "Ten green bottles hanging on the wall,\n",
            "Ten green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be nine green bottles hanging on the wall.\n",
            "\n",
            "Nine green bottles hanging on the wall,\n",
            "Nine green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be eight green bottles hanging on the wall.\n",
            "\n",
            "Eight green bottles hanging on the wall,\n",
            "Eight green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be seven green bottles hanging on the wall.\n",
            "\n",
            "Seven green bottles hanging on the wall,\n",
            "Seven green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be six green bottles hanging on the wall.\n",
            "\n",
            "Six green bottles hanging on the wall,\n",
            "Six green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be five green bottles hanging on the wall.\n",
            "\n",
            "Five green bottles hanging on the wall,\n",
            "Five green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be four green bottles hanging on the wall.\n",
            "\n",
            "Four green bottles hanging on the wall,\n",
            "Four green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be three green bottles hanging on the wall.\n",
            "\n",
            "Three green bottles hanging on the wall,\n",
            "Three green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be two green bottles hanging on the wall.\n",
            "\n",
            "Two green bottles hanging on the wall,\n",
            "Two green bottles hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be one green bottle hanging on the wall.\n",
            "\n",
            "One green bottle hanging on the wall,\n",
            "One green bottle hanging on the wall,\n",
            "And if one green bottle should accidentally fall,\n",
            "There'll be no green bottles hanging on the wall.",
        )
    );
}
