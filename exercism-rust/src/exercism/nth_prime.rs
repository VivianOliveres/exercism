pub fn nth(n: u32) -> u32 {
    let n = n as usize; // Cast only once
    let mut prime_numbers: Vec<u32> = Vec::with_capacity(n as usize + 1); // initialized capacity to avoid re-allocation
    prime_numbers.push(2); // First prime number

    let mut i: u32 = 3;
    while prime_numbers.len() <= n {
        let limit = (i as f64).sqrt() as u32;
        if !prime_numbers
            .iter()
            .take_while(|&&x| x <= limit) // Optimization for prime numbers: cannot be more than sqrt(i)
            .any(|&x| i % x == 0) {
            prime_numbers.push(i);
        }
        i += 2 // Skip even numbers
    }
    prime_numbers[n]
}

#[test]
fn first_prime() {
    let output = nth(0);
    let expected = 2;
    assert_eq!(output, expected);
}

#[test]
fn second_prime() {
    let output = nth(1);
    let expected = 3;
    assert_eq!(output, expected);
}

#[test]
fn sixth_prime() {
    let output = nth(5);
    let expected = 13;
    assert_eq!(output, expected);
}

#[test]
fn big_prime() {
    let output = nth(10_000);
    let expected = 104_743;
    assert_eq!(output, expected);
}
