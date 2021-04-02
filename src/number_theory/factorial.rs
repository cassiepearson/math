//! Factorial Trait
use crate::number_theory::numbers::Integers;
use anyhow::{anyhow, Result};

/// Implement the factorial operation
pub trait Factorial<T = Self> {
    fn factorial(&self) -> Result<T>;
}

impl<T: Integers> Factorial<T> for T {
    fn factorial(&self) -> Result<T> {
        let mut i = T::one();
        let mut acc = T::one();
        while i < *self {
            i += T::one();
            match acc.checked_mul(&i) {
                Some(new) => acc = new,
                None => return Err(anyhow!("Factorial failed to successfully compute.")),
            }
        }
        Ok(acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    #[case(4, 24)]
    #[case(11, 39916800)]
    fn usize_factorial_test(#[case] a: usize, #[case] expected: usize) {
        // Unwrapping because must test guarantee this will not err on low values
        assert_eq!(expected, a.factorial().unwrap())
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    #[case(4, 24)]
    #[case(11, 39916800)]
    fn isize_factorial_test(#[case] a: isize, #[case] expected: isize) {
        // Unwrapping because must test guarantee this will not err on low values
        assert_eq!(expected, a.factorial().unwrap())
    }
}
