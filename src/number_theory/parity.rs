//! Parity Trait
use crate::number_theory::numbers::Integers;

/// Implement parity checking
pub trait Parity<T> {
    fn parity(self) -> String;
    fn is_odd(self) -> bool;
    fn is_even(self) -> bool;
}

impl<T> Parity<T> for T
where
    T: Integers,
{
    fn parity(self) -> String {
        if self.is_even() {
            "even".to_string()
        } else {
            "odd".to_string()
        }
    }

    fn is_odd(self) -> bool {
        if self % (T::one() + T::one()) != T::zero() {
            true
        } else {
            false
        }
    }

    fn is_even(self) -> bool {
        if self % (T::one() + T::one()) == T::zero() {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, false)]
    #[case(2, true)]
    #[case(3, false)]
    #[case(300, true)]
    #[case(301, false)]
    fn usize_is_even_test(#[case] a: usize, #[case] expected: bool) {
        assert_eq!(expected, a.is_even())
    }

    #[rstest]
    #[case(1, false)]
    #[case(2, true)]
    #[case(3, false)]
    #[case(300, true)]
    #[case(301, false)]
    #[case(-1, false)]
    #[case(-2, true)]
    #[case(-3, false)]
    #[case(-300, true)]
    #[case(-301, false)]
    fn isize_is_even_test(#[case] a: isize, #[case] expected: bool) {
        assert_eq!(expected, a.is_even())
    }

    #[rstest]
    #[case(1, true)]
    #[case(2, false)]
    #[case(3, true)]
    #[case(300, false)]
    #[case(301, true)]
    fn usize_is_odd_test(#[case] a: usize, #[case] expected: bool) {
        assert_eq!(expected, a.is_odd())
    }

    #[rstest]
    #[case(1, true)]
    #[case(2, false)]
    #[case(3, true)]
    #[case(300, false)]
    #[case(301, true)]
    #[case(-1, true)]
    #[case(-2, false)]
    #[case(-3, true)]
    #[case(-300, false)]
    #[case(-301, true)]
    fn isize_is_odd_test(#[case] a: isize, #[case] expected: bool) {
        assert_eq!(expected, a.is_odd())
    }
}
