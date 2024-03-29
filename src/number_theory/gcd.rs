//! Greatest Common Divisor Trait
use crate::general::numbers::Integer;

/// Greatest Common Divisor
///
/// This trait contains multiple function implementations of algorithms for finding the greatest common divisor
pub trait Gcd<T> {
    fn gcd(self, other: T) -> T;
    fn gcd_subtraction(self, other: T) -> T;
}
#[macro_export]
macro_rules! gcd {
    ($t: ident) => {
        impl<T> Gcd<T> for T
        where
            T: $t,
        {
            /// Greatest Commmon Divisor
            ///
            /// Find the GCD of two numbers using Euclidean division (Euclidean algorithm standard form)
            fn gcd(self, other: T) -> T {
                let mut a = self;
                let mut b = other;
                while b != T::zero() {
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
            fn gcd_subtraction(self, other: T) -> T {
                let mut a = self;
                let mut b = other;
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
        }
    };
}
gcd!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1073, 29, 29)]
    #[case(1073, 25, 1)]
    fn usize_gcd_test(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        assert_eq!(expected, a.gcd(b))
    }

    #[rstest]
    #[case(1073, 29, 29)]
    #[case(1073, 25, 1)]
    fn isize_gcd_test(#[case] a: isize, #[case] b: isize, #[case] expected: isize) {
        assert_eq!(expected, a.gcd(b))
    }

    #[rstest]
    #[case(1073, 29, 29)]
    #[case(1073, 25, 1)]
    fn usize_gcd_subtraction_test(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        assert_eq!(expected, a.gcd_subtraction(b))
    }

    #[rstest]
    #[case(1073, 29, 29)]
    #[case(1073, 25, 1)]
    fn isize_gcd_subtraction_test(#[case] a: isize, #[case] b: isize, #[case] expected: isize) {
        assert_eq!(expected, a.gcd_subtraction(b))
    }
}
