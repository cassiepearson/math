//! Euler Totient Trait

use crate::{general::numbers::Integer, number_theory::primality::Primality};

/// Euler Totient Function
///
/// Return the value of the Euler Totient Function:
/// http://mathworld.wolfram.com/TotientFunction.html
pub trait EulerTotient<T> {
    fn euler_totient(self) -> T;
    fn primality_checked_euler_totient(self) -> T;
}

#[macro_export]
macro_rules! euler_totient {
    ($t: ident) => {
        impl<T: $t> EulerTotient<T> for T {
            fn euler_totient(self) -> T {
                let one = T::one();
                let mut count = T::zero();
                let mut i = one;
                while i < self {
                    if self.relative_primality(i) {
                        count += one;
                    }
                    i += one;
                }
                count
            }

            /// Standard Euler totient but after a primality check for the trivial case
            ///
            /// Results will be the same as for the standard totient, but I was curious about benchmarking for when primes may exist in the checked set of numbers
            fn primality_checked_euler_totient(self) -> T {
                let one = T::one();
                if self.primality() {
                    self - one
                } else {
                    let mut count = T::zero();
                    let mut i = one;
                    while i < self {
                        if self.relative_primality(i) {
                            count += one;
                        }
                        i += one;
                    }
                    count
                }
            }
        }
    };
}

euler_totient!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(5, 4)]
    #[case(9, 6)]
    #[case(200, 80)]
    fn usize_euler_totient_test(#[case] a: usize, #[case] expected: usize) {
        assert_eq!(expected, a.euler_totient())
    }

    #[rstest]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(5, 4)]
    #[case(9, 6)]
    #[case(200, 80)]
    fn isize_euler_totient_test(#[case] a: isize, #[case] expected: isize) {
        assert_eq!(expected, a.euler_totient())
    }
    #[rstest]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(5, 4)]
    #[case(9, 6)]
    #[case(200, 80)]
    fn usize_primality_checked_euler_totient_test(#[case] a: usize, #[case] expected: usize) {
        assert_eq!(expected, a.primality_checked_euler_totient())
    }

    #[rstest]
    #[case(2, 1)]
    #[case(3, 2)]
    #[case(5, 4)]
    #[case(9, 6)]
    #[case(200, 80)]
    fn isize_primality_checked_euler_totient_test(#[case] a: isize, #[case] expected: isize) {
        assert_eq!(expected, a.primality_checked_euler_totient())
    }
}
