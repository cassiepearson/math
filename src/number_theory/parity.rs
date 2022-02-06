//! Parity Trait
use crate::general::numbers::Integer;

/// Implement parity checking
pub trait Parity<T> {
    fn parity(self) -> NumParity;
    fn is_odd(self) -> bool;
    fn is_even(self) -> bool;
}

pub enum NumParity {
    Even,
    Odd,
}

#[macro_export]
macro_rules! impl_parity {
    ($t: ident) => {
        impl<T> Parity<T> for T
        where
            T: $t,
        {
            fn parity(self) -> NumParity {
                if self.is_even() {
                    NumParity::Even
                } else {
                    NumParity::Odd
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
    };
}
impl_parity!(Integer);

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
