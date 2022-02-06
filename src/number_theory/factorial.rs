//! Factorial Trait
use crate::general::numbers::Integer;
use anyhow::{anyhow, Result};

/// Implement the factorial operation
pub trait Factorial<T = Self> {
    fn factorial(&self) -> Result<T>;
}

#[macro_export]
macro_rules! impl_factorial {
    ($t: ident) => {
        impl<T: $t> Factorial<T> for T {
            #[inline]
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
    };
}

impl_factorial!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use test::test::Bencher;

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

    #[bench]
    fn bench_factorial(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(12i32);
            (0..n).fold(0, |a, b| b.factorial().unwrap());
        });
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
