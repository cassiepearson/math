//! Modular Multiplicative Inverse: ax = 1 (mod b)
use crate::{
    general::{errors::NumberTheoryErr, numbers::Integer},
    number_theory::euclidean_algs::ExtendedEuclidean,
};
use std::fmt::Display;

type Result<T> = std::result::Result<T, NumberTheoryErr>;

/// Implement the modular multiplicative inverse operation
pub trait MultiplicativeInverse<T> {
    fn multiplicative_inverse(&self, b: T) -> Result<T>;
}

#[macro_export]
macro_rules! multiplicative_inverse {
    ($t: ident) => {
        impl<T: $t + Display> MultiplicativeInverse<T> for T {
            /// Modular Multiplicative Inverse
            ///
            /// This function returns the modular multiplicative inverse of an integer.
            fn multiplicative_inverse(&self, other: T) -> Result<T> {
                // Calculate the GCD and Bezout coefficients
                let egcd = self.egcd(other);
                if egcd.0 != T::one() {
                    Err(NumberTheoryErr::InverseDNE(format!(
                        "No modular multiplicative inverse exists {} mod {}",
                        self, other
                    )))
                } else {
                    Ok(egcd.1)
                }
            }
        }
    };
}

multiplicative_inverse!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        case(1, 1, 0),
        case(9, 26, 3),
        case(1073, 25, 12),
        #[should_panic]
        case(1073, 29, 0)
    )]
    fn isize_multiplicative_inverse_test(
        #[case] a: isize,
        #[case] b: isize,
        #[case] expected: isize,
    ) {
        assert_eq!(expected, a.multiplicative_inverse(b).unwrap())
    }
}
