use crate::general::numbers::Integer;

/// Generalized Continued Fraction
///
/// Returns the set of integer parts of the continued fraction
/// https://en.wikipedia.org/wiki/Continued_fraction
/// https://mathworld.wolfram.com/ContinuedFraction.html
pub trait ContinuedFraction<T> {
    fn continued_frac(self, other: T) -> Vec<T>;
}

#[macro_export]
macro_rules! continued_fraction {
    ($t: ident) => {
        impl<T: $t> ContinuedFraction<T> for T {
            fn continued_frac(self, other: T) -> Vec<T> {
                // Compute the gcd, store the information on the quotients, build the continued fraction
                let mut fraction = vec![];
                let mut i;
                let mut temp;
                let mut a = self;
                let mut b = other;
                loop {
                    // Find the integer portion of the fraction
                    i = a / b;
                    fraction.push(i);

                    // Subtract the integer portion of the fraction
                    a -= (b * i);
                    if a == T::zero() {
                        break;
                    }

                    // Take the reciprocal of the fraction
                    temp = b;
                    b = a;
                    a = temp;
                }

                // Return continued fraction to the user
                fraction
            }
        }
    };
}

continued_fraction!(Integer);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(649, 200, vec![3, 4, 12, 4])]
    #[case(43, 19, vec![2, 3, 1, 4])]
    #[case(19, 43, vec![0, 2, 3, 1, 4])]
    #[case(7, 3, vec![2, 3])]
    #[case(3, 7, vec![0, 2, 3])]
    fn usize_continued_fraction_test(
        #[case] a: usize,
        #[case] b: usize,
        #[case] expected: Vec<usize>,
    ) {
        assert_eq!(expected, a.continued_frac(b))
    }

    #[rstest]
    #[case(649, 200, vec![3, 4, 12, 4])]
    #[case(43, 19, vec![2, 3, 1, 4])]
    #[case(19, 43, vec![0, 2, 3, 1, 4])]
    #[case(7, 3, vec![2, 3])]
    #[case(3, 7, vec![0, 2, 3])]
    fn isize_continued_fraction_test(
        #[case] a: isize,
        #[case] b: isize,
        #[case] expected: Vec<isize>,
    ) {
        assert_eq!(expected, a.continued_frac(b))
    }
}
