//! Primality Trait
use crate::{
    general::{errors::MathLibError, numbers::Integer},
    number_theory::{factorial::Factorial, gcd::Gcd},
};
use std::fmt::Display;

type Result<T> = std::result::Result<T, MathLibError>;

/// Primality Trait
///
/// Implements various functions covering primality tests and prime factorizations
pub trait Primality<T> {
    fn primality(self) -> bool;
    fn relative_primality(self, other: T) -> bool;
    fn prime_factorization(self) -> Vec<T>;
}

/// Wilson's Primality Trait
///
/// Implements factorial-based primality tests
pub trait WilsonsPrimality<T> {
    fn wilson(self) -> Result<bool>;
}

#[macro_export]
macro_rules! impl_primality {
    ($t: ident) => {
        impl<T> Primality<T> for T
        where
            T: $t,
        {
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
    };
}

#[macro_export]
macro_rules! impl_wilsons_primality {
    ($t: ident) => {
        impl<T> WilsonsPrimality<T> for T
        where
            T: $t + Display,
        {
            /// Wilson's Theorem to test Primality
            ///
            /// Only use this if you are using small integers. Actually, don't use this at all. Wilson's theorem is never practical to compute but has implications in theory
            fn wilson(self) -> Result<bool> {
                // Rust can perform a remainder operation, but not a modular operation
                match (self - T::one()).factorial() {
                    Ok(val) => Ok((val % self) + T::one() + T::one() == ((T::one()) % self) + self),
                    Err(error) => Err(error),
                }
            }
        }
    };
}

impl_primality!(Integer);
impl_wilsons_primality!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        case::prime_1(2, true), 
        case::prime_2(3, true), 
        case::prime_3(5, true),
        case::non_prime_1(15, false),
        case::non_prime_2(4, false),
        case::non_prime_3(9, false),
        #[should_panic]
        case::should_panic_1(22, false),
        #[should_panic]
        case::should_panic_2(29, true)
    )]
    fn usize_wilson_test(#[case] a: usize, #[case] expected: bool) {
        assert_eq!(expected, a.wilson().unwrap())
    }

    #[rstest(
        case::prime_1(2, true), 
        case::prime_2(3, true), 
        case::prime_3(5, true),
        case::non_prime_1(15, false),
        case::non_prime_2(4, false),
        case::non_prime_3(9, false),
        #[should_panic]
        case::should_panic_1(22, false),
        #[should_panic]
        case::should_panic_2(29, true)
    )]
    fn isize_wilson_test(#[case] a: isize, #[case] expected: bool) {
        assert_eq!(expected, a.wilson().unwrap())
    }

    #[test]
    #[should_panic]
    fn wilson_overflow_test() {
        1000.wilson().unwrap();
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
