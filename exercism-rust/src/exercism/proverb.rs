use std::fmt::Write;

pub fn build_proverb(list: &[&str]) -> String {
    let mut result: String = String::new();
    if list.is_empty() {
        return result;
    }

    for i in 1..list.len() {
        writeln!(result, "For want of a {} the {} was lost.", list[i-1], list[i]).unwrap();
    }

    // Use of `window` method
    //
    // for pair in list.windows(2) {
    //     writeln!(result, "For want of a {} the {} was lost.", pair[0], pair[1]).unwrap();
    // }

    // FP version with `fold`
    // Not recommended as `fold` is doing a edge effect (ie write into `results`).
    //
    // list.iter().skip(1).fold(list[0], |previous, current| {
    //     writeln!(result, "For want of a {} the {} was lost.", previous, current).unwrap();
    //     current
    // } );

    write!(result, "And all for the want of a {}.", list[0]).unwrap();
    result
}

#[test]
fn zero_pieces() {
    let input = &[];
    let output = build_proverb(input);
    let expected = String::new();
    assert_eq!(output, expected);
}

#[test]
fn one_piece() {
    let input = &["nail"];
    let output = build_proverb(input);
    let expected: String = ["And all for the want of a nail."].join("\n");
    assert_eq!(output, expected);
}

#[test]
fn two_pieces() {
    let input = &["nail", "shoe"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[test]
fn three_pieces() {
    let input = &["nail", "shoe", "horse"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[test]
fn full_proverb() {
    let input = &[
        "nail", "shoe", "horse", "rider", "message", "battle", "kingdom",
    ];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a nail the shoe was lost.",
        "For want of a shoe the horse was lost.",
        "For want of a horse the rider was lost.",
        "For want of a rider the message was lost.",
        "For want of a message the battle was lost.",
        "For want of a battle the kingdom was lost.",
        "And all for the want of a nail.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}

#[test]
fn four_pieces_modernized() {
    let input = &["pin", "gun", "soldier", "battle"];
    let output = build_proverb(input);
    let expected: String = [
        "For want of a pin the gun was lost.",
        "For want of a gun the soldier was lost.",
        "For want of a soldier the battle was lost.",
        "And all for the want of a pin.",
    ]
    .join("\n");
    assert_eq!(output, expected);
}
