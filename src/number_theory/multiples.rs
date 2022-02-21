//! Multiples Functions
use crate::general::numbers::Integer;

#[macro_export]
macro_rules! impl_multiples {
    ($t: ident) => {
        /// Multiples
        ///
        /// Vector of all multiples of the input factors that are strictly below the max value
        #[allow(dead_code)]
        pub fn multiples<T>(factors: Vec<T>, max: T) -> Vec<T>
        where
            T: $t,
        {
            let mut multiples = vec![];
            for factor in factors {
                let mut i = T::one();
                while i <= (max / factor) {
                    let current = factor * i;
                    if current < max {
                        multiples.push(current);
                    }
                    i += T::one();
                }
            }
            multiples.sort();
            multiples.dedup();
            multiples
        }

        /// Sum of multiples
        ///
        /// Sum all multiples of the input factors that are below the max value
        #[allow(dead_code)]
        pub fn sum_of_multiples<T>(factors: Vec<T>, max: T) -> T
        where
            T: $t,
        {
            let mut out = T::zero();
            for multiple in multiples(factors, max) {
                out += multiple;
            }
            out
        }
    };
}

impl_multiples!(Integer);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples() {
        assert_eq!(multiples(vec![3, 5], 10), vec![3, 5, 6, 9]);
        assert_eq!(
            multiples(vec![2, 5], 20),
            vec![2, 4, 5, 6, 8, 10, 12, 14, 15, 16, 18]
        )
    }

    #[test]
    fn test_sum_of_multiples() {
        assert_eq!(sum_of_multiples(vec![3, 5], 10), 23);
        assert_eq!(sum_of_multiples(vec![2, 5], 20), 110)
    }
}
