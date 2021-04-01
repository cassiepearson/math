//! Primality Trait
use crate::number_theory::{factorial::Factorial, gcd::GCD, numbers::Integers};

/// Primality Trait
///
/// Implements various functions covering primality tests and prime factorizations
pub trait Primality<T> {
    fn wilson(self) -> bool;
    fn primality(self) -> bool;
    fn relative_primality(self, other: T) -> bool;
    fn prime_factorization(self) -> Vec<T>;
}

impl<T> Primality<T> for T
where
    T: Integers,
{
    /// Wilson's Theorem to test Primality
    ///
    /// Only use this if you are using small integers. Actually, don't use this at all. Wilson's theorem is never practical to compute but has implications in theory
    fn wilson(self) -> bool {
        // Rust can perform a remainder operation, but not a modular operation
        ((self - T::one()).factorial() % self) + T::one() + T::one() == ((T::one()) % self) + self
    }

    /// Primality
    ///
    /// Check if a number is prime
    fn primality(self) -> bool {
        let zero = T::zero();
        let one = T::one();
        let two = T::one() + T::one();
        let three = two + T::one();
        if self <= three {
            return (self > one);
        } else if self % two == zero || self % three == zero {
            return false;
        }

        let mut b = three + T::one() + T::one();
        while b * b <= self {
            if self % b == zero || self % (b + two) == zero {
                return false;
            }
            b = b + three + three;
        }

        true
    }

    /// Relative Primality (Coprimality)
    ///
    /// Check if two numbers are relatively prime
    fn relative_primality(self, b: T) -> bool {
        self.gcd(b) == T::one()
    }

    /// Prime factorization
    ///
    /// Return the prime factorization of the input integer
    fn prime_factorization(self) -> Vec<T> {
        // Initialize
        let mut n = self;
        let zero = T::zero();
        let two = T::one() + T::one();
        let three = T::one() + T::one() + T::one();
        let mut output: Vec<T> = vec![];

        // While even, divide out 2
        while n % two == zero {
            output.push(two);
            n /= two;
        }

        // Odd numbers greater than n
        let limit = n / two;
        let mut x = three;
        while x <= limit {
            while n % x == zero {
                output.push(x);
                n /= x;
            }
            x += two;
        }

        if n > two {
            output.push(n);
        }

        // Return
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, true)]
    #[case(3, true)]
    #[case(5, true)]
    #[case(15, false)]
    #[case(30, false)]
    #[case(225, false)]
    fn usize_wilson_test(#[case] a: usize, #[case] expected: bool) {
        assert_eq!(expected, a.wilson())
    }

    #[rstest]
    #[case(2, true)]
    #[case(3, true)]
    #[case(5, true)]
    #[case(15, false)]
    #[case(30, false)]
    #[case(225, false)]
    fn isize_wilson_test(#[case] a: isize, #[case] expected: bool) {
        assert_eq!(expected, a.wilson())
    }

    #[rstest]
    #[case(2, true)]
    #[case(3, true)]
    #[case(5, true)]
    #[case(15, false)]
    #[case(30, false)]
    #[case(225, false)]
    fn usize_primality_test(#[case] a: usize, #[case] expected: bool) {
        assert_eq!(expected, a.primality())
    }

    #[rstest]
    #[case(2, true)]
    #[case(3, true)]
    #[case(5, true)]
    #[case(15, false)]
    #[case(30, false)]
    #[case(225, false)]
    fn isize_primality_test(#[case] a: isize, #[case] expected: bool) {
        assert_eq!(expected, a.primality())
    }

    #[rstest]
    #[case(2, vec![2])]
    #[case(3, vec![3])]
    #[case(4, vec![2, 2])]
    #[case(5, vec![5])]
    #[case(6, vec![2, 3])]
    #[case(9, vec![3, 3])]
    #[case(315, vec![3, 3, 5, 7])]
    fn usize_prime_factorization_test(#[case] a: usize, #[case] expected: Vec<usize>) {
        assert_eq!(expected, a.prime_factorization())
    }

    #[rstest]
    #[case(2, vec![2])]
    #[case(3, vec![3])]
    #[case(4, vec![2, 2])]
    #[case(5, vec![5])]
    #[case(6, vec![2, 3])]
    #[case(9, vec![3, 3])]
    #[case(315, vec![3, 3, 5, 7])]
    fn isize_prime_factorization_test(#[case] a: isize, #[case] expected: Vec<isize>) {
        assert_eq!(expected, a.prime_factorization())
    }
}
