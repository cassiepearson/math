//! Factorial Trait
use crate::number_theory::numbers::Integers;

/// Implement the factorial operation
pub trait Factorial<T = Self> {
    fn factorial(&self) -> T;
}

impl<T: Integers> Factorial<T> for T {
    fn factorial(&self) -> T {
        let mut i = T::one();
        let mut acc = T::one();
        while i < *self {
            i += T::one();
            acc *= i;
        }
        acc
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
        assert_eq!(expected, a.factorial())
    }

    #[rstest]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 6)]
    #[case(4, 24)]
    #[case(11, 39916800)]
    fn isize_factorial_test(#[case] a: isize, #[case] expected: isize) {
        assert_eq!(expected, a.factorial())
    }
}
