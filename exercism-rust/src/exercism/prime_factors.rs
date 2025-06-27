// This solution does NOT compute all prime numbers in order to test them (which would be inefficient for large prime numbers).
// It stops after sqrt(n) (ie `candidate * candidate <= n`) as after this value there are no other prime numbers except n.
// If the rest (ie n) is not 1 then rest is the last prime number.
pub fn factors(mut n: u64) -> Vec<u64> {
    let mut results = Vec::new();
    let mut candidate = 2;

    while candidate * candidate <= n { // Test to sqrt(n)
        while n % candidate == 0 { // Test every candidate, multiple times
            results.push(candidate);
            n /= candidate;
        }
        candidate += if candidate == 2 { 1 } else { 2 }; // skip even numbers after 2
    }

    if n > 1 {
        results.push(n);
    }

    results
}

#[test]
fn no_factors() {
    let factors = factors(1);
    let expected = [];
    assert_eq!(factors, expected);
}

#[test]
fn prime_number() {
    let factors = factors(2);
    let expected = [2];
    assert_eq!(factors, expected);
}

#[test]
fn another_prime_number() {
    let factors = factors(3);
    let expected = [3];
    assert_eq!(factors, expected);
}

#[test]
fn square_of_a_prime() {
    let factors = factors(9);
    let expected = [3, 3];
    assert_eq!(factors, expected);
}

#[test]
fn product_of_first_prime() {
    let factors = factors(4);
    let expected = [2, 2];
    assert_eq!(factors, expected);
}

#[test]
fn cube_of_a_prime() {
    let factors = factors(8);
    let expected = [2, 2, 2];
    assert_eq!(factors, expected);
}

#[test]
fn product_of_second_prime() {
    let factors = factors(27);
    let expected = [3, 3, 3];
    assert_eq!(factors, expected);
}

#[test]
fn product_of_third_prime() {
    let factors = factors(625);
    let expected = [5, 5, 5, 5];
    assert_eq!(factors, expected);
}

#[test]
fn product_of_first_and_second_prime() {
    let factors = factors(6);
    let expected = [2, 3];
    assert_eq!(factors, expected);
}

#[test]
fn product_of_primes_and_non_primes() {
    let factors = factors(12);
    let expected = [2, 2, 3];
    assert_eq!(factors, expected);
}

#[test]
fn product_of_primes() {
    let factors = factors(901_255);
    let expected = [5, 17, 23, 461];
    assert_eq!(factors, expected);
}

#[test]
fn factors_include_a_large_prime() {
    let factors = factors(93_819_012_551);
    let expected = [11, 9_539, 894_119];
    assert_eq!(factors, expected);
}
