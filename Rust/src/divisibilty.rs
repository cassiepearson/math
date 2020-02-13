//! Divisibility Functions
// Author: Christopher Negrich
// Github: cnegrich
//
// Some functions on divisibility

/// Modulus
///
/// Rust can perform the remainder operation, but not a true modulo
fn modulo(a: isize, b: isize) -> isize {
    (a % b) + b
}

/// Factorial
fn factorial(a: isize) -> isize {
    (1..=a).fold(1, |product, i| product * i)
}

/// Wilson's Theorem to test Primality
///
/// Only use this if you are using small integers
fn wilson(p: isize) -> bool {
    // Rust can perform a remainder operation, but not a modular operation
    (factorial(p - 1) % p) == ((-1 % p) + p)
}

/// Primality
///
/// Check if a number is prime
fn primality(a: usize) -> bool {
    if a <= 3 {
        return (a > 1);
    } else if a % 2 == 0 || a % 3 == 0 {
        return false;
    }

    let mut b = 5;
    while b * b <= a {
        if a % b == 0 || a % (b + 2) == 0 {
            return false;
        }
        b = b + 6;
    }

    true
}

/// Relative Primality (Coprimality)
///
/// Check if two numbers are relatively prime
fn relative_primality(a: usize, b: usize) -> bool {
    match gcd(a, b) {
        1 => true,
        _ => false,
    }
}

/// Greatest Commmon Divisor
///
/// Find the GCD of two numbers using Euclidean division (Euclidean algorithm standard form)
fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }

    // Return a
    a
}

/// Greatest Commmon Divisor
///
/// Find the GCD of two numbers using a subtraction based method
fn gcd_subtraction(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    // Return a
    a
}

/// Greatest Commmon Divisor
///
/// Find the GCD of two numbers using a recursive methodology
fn gcd_recursive(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

/// Continued Fraction
///
/// Find the continued fraction, print in standard notation, return as a vector
fn continued_fraction(mut a: usize, mut b: usize) -> Vec<usize> {
    // Compute the gcd, store the information on the quotients, build the continued fraction
    let mut fraction: Vec<usize> = vec![];
    let mut i = 0;
    let mut temp = 0;
    loop {
        // Find the integer portion of the fraction
        i = (a as f64 / b as f64).floor() as usize;
        fraction.push(i);

        // Subtract the integer portion of the fraction
        a = a - (b * i);
        if a == 0 {
            break;
        }

        // Take the reciprocal of the fraction
        temp = b;
        b = a;
        a = temp;
    }

    // Return continued fraction to the user
    fraction
}

/// Prime factorization
///
/// Return the prime factorization of the input integer
fn prime_factorization(mut n: usize) -> Vec<usize> {
    // Initialize output
    let mut output: Vec<usize> = vec![];

    // While even, divide out 2
    while n % 2 == 0 {
        output.push(2);
        n /= 2;
    }

    // Each the odd numbers greater than n
    let limit: usize = (n as f64).sqrt().floor() as usize;
    for x in (3..=limit).step_by(2) {
        while n % x == 0 {
            output.push(x);
            n /= x;
        }
    }

    if n > 2 {
        output.push(n);
    }

    // Return
    output
}

// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 5 * 4 * 3 * 2 * 1);
        assert_eq!(factorial(15), 1307674368000);
    }

    #[test]
    fn test_wilson() {
        assert_eq!(wilson(5), true);
        assert_eq!(wilson(2), true);
        assert_eq!(wilson(15), false);
        assert_eq!(wilson(30), false);
    }

    #[test]
    fn test_primality() {
        assert_eq!(primality(5), true);
        assert_eq!(primality(2), true);
        assert_eq!(primality(15), false);
        assert_eq!(primality(222), false);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(1073, 29), 29);
        assert_eq!(gcd(1073, 25), 1);
    }

    #[test]
    fn test_gcd_subtraction() {
        assert_eq!(gcd_subtraction(1073, 29), 29);
        assert_eq!(gcd_subtraction(1073, 25), 1);
    }

    #[test]
    fn test_gcd_recursive() {
        assert_eq!(gcd_recursive(1073, 29), 29);
        assert_eq!(gcd_recursive(1073, 25), 1);
    }

    #[test]
    fn test_continued_fraction() {
        assert_eq!(continued_fraction(649, 200), vec![3, 4, 12, 4]);
        assert_eq!(continued_fraction(43, 19), vec![2, 3, 1, 4]);
        assert_eq!(continued_fraction(19, 43), vec![0, 2, 3, 1, 4]);
        assert_eq!(continued_fraction(7, 3), vec![2, 3]);
        assert_eq!(continued_fraction(3, 7), vec![0, 2, 3]);
    }

    #[test]
    fn test_prime_factorization() {
        assert_eq!(prime_factorization(315), vec![3, 3, 5, 7]);
        assert_eq!(prime_factorization(3), vec![3]);
        assert_eq!(prime_factorization(4), vec![2, 2]);
    }
}
