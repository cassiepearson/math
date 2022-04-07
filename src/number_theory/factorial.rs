//! Factorial Trait
use crate::general::{errors::MathLibError, numbers::Integer};
use std::fmt::Display;

type Result<T> = std::result::Result<T, MathLibError>;

/// Implement the factorial operation
pub trait Factorial<T = Self> {
    fn factorial(&self) -> Result<T>;
}

#[macro_export]
macro_rules! impl_factorial {
    ($t: ident) => {
        impl<T: $t + Display> Factorial<T> for T {
            #[inline]
            fn factorial(&self) -> Result<T> {
                let mut i = T::one();
                let mut acc = T::one();
                while i < *self {
                    i += T::one();
                    match acc.checked_mul(&i) {
                        Some(new) => acc = new,
                        None => {
                            return Err(MathLibError::NumberTheoryErr(format!(
                                "Factorial overflow at: {}",
                                acc
                            )))
                        }
                    }
                }
                Ok(acc)
            }
        }
    };
}

impl_factorial!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(
        case::one(1, 1),
        case::two(2, 2),
        case::three(3, 6),
        case::four(4, 24),
        case::eleven(11, 39916800),
        case::twelve(12, 479001600),
        case::thirteen(13, 6227020800),
        #[should_panic]
        case::fourteen(14, 14302774849602060)
    )]
    fn usize_factorial_test(#[case] a: usize, #[case] expected: usize) {
        assert_eq!(expected, a.factorial().unwrap())
    }

    #[rstest(
        case::one(1, 1),
        case::two(2, 2),
        case::three(3, 6),
        case::four(4, 24),
        case::eleven(11, 39916800),
        case::twelve(12, 479001600),
        case::thirteen(13, 6227020800),
        #[should_panic]
        case::fourteen(14, 14302774849602060)
    )]
    fn isize_factorial_test(#[case] a: isize, #[case] expected: isize) {
        // Unwrapping because must test guarantee this will not err on low values
        assert_eq!(expected, a.factorial().unwrap())
    }
}
