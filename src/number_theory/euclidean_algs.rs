//! Euclidean Algorithms Traits
use crate::general::numbers::Integer;

/// Euclidean algorithm
///
/// This trait contains the standard form of the euclidean algorithm. It is equivalent to the euclidean implementation.
pub trait Euclidean<T> {
    fn euclidean(self, other: T) -> T;
}
#[macro_export]
macro_rules! impl_euclidean {
    ($t: ident) => {
        impl<T> Euclidean<T> for T
        where
            T: $t,
        {
            /// Greatest Commmon Divisor
            ///
            /// Find the euclidean of two numbers using Euclidean division (Euclidean algorithm standard form)
            fn euclidean(self, other: T) -> T {
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
        }
    };
}

/// Extended Euclidean algorithm
///
/// Solves the equation ax + by = gcd(a, b) for x,y given a,b. Bezout's lemma guarantees solvability and x,y are called Bezout's coefficients.
pub trait ExtendedEuclidean<T> {
    fn extended_euclidean(self, other: T) -> (T, T, T, T, T);
    fn egcd(self, other: T) -> (T, T, T);
    fn bezout(self, other: T) -> (T, T);
}
#[macro_export]
macro_rules! impl_extended_euclidean {
    ($t: ident) => {
        impl<T> ExtendedEuclidean<T> for T
        where
            T: $t,
        {
            /// Extended Euclidean Algorithm
            ///
            /// This function returns all calculated pieces of the algorithm: gcd, bezout_x, bezout_y, quotient_x, quotient_y
            fn extended_euclidean(self, other: T) -> (T, T, T, T, T) {
                // Initialize
                let (mut x, mut y, mut prev_x, mut prev_y, mut rem, mut prev_rem) =
                    (T::zero(), T::one(), T::one(), T::zero(), other, self);

                // Loop - induct until we have found the remainder of 0
                while rem != T::zero() {
                    // Quotient
                    let q = prev_rem / rem;

                    // Update
                    (prev_rem, rem) = (rem, prev_rem - q * rem);
                    (prev_x, x) = (x, prev_x - q * x);
                    (prev_y, y) = (y, prev_y - q * y);
                }

                // Return gcd, bezout_x, bezout_y, quotient_x, quotient_y
                (prev_rem, prev_x, prev_y, x, y)
            }

            /// Extended Greatest Common Divisor
            ///
            /// This function calls the extended_euclidean function and returns only the gcd and Bezout's coefficients.
            fn egcd(self, other: T) -> (T, T, T) {
                let out = self.extended_euclidean(other);

                // Return only the GCD and Bezout coefficients
                (out.0, out.1, out.2)
            }

            /// Bezout Coefficients
            ///
            /// This function calls the extended_euclidean function and returns only the Bezout coefficients.
            fn bezout(self, other: T) -> (T, T) {
                let out = self.extended_euclidean(other);

                // Return only the Bezout coefficients
                (out.1, out.2)
            }
        }
    };
}
impl_euclidean!(Integer);
impl_extended_euclidean!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1073, 29, 29)]
    #[case(1073, 25, 1)]
    fn usize_euclidean_test(#[case] a: usize, #[case] b: usize, #[case] expected: usize) {
        assert_eq!(expected, a.euclidean(b))
    }

    #[rstest]
    #[case(1073, 29, 29)]
    #[case(1073, 25, 1)]
    fn isize_euclidean_test(#[case] a: isize, #[case] b: isize, #[case] expected: isize) {
        assert_eq!(expected, a.euclidean(b))
    }

    #[rstest]
    #[case(1073, 29, (29, 0, 1))]
    #[case(1073, 25, (1, 12, -515))]
    fn isize_egcd_test(
        #[case] a: isize,
        #[case] b: isize,
        #[case] expected: (isize, isize, isize),
    ) {
        assert_eq!(expected, a.egcd(b))
    }
}
