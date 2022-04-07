//! Multiples Functions
use crate::general::numbers::Integer;

#[macro_export]
macro_rules! impl_multiples {
    ($t: ident) => {
        /// Multiples
        ///
        /// Vector of all unique multiples of a number that fit within the range
        /// [factor, maximum representable number for the type]
        #[allow(dead_code)]
        pub fn multiples_of<T>(factor: T) -> Vec<T>
        where
            T: $t,
        {
            let mut multiples = vec![];
            let mut i = T::one();
            'find: loop {
                let current = factor.checked_mul(&i);
                if current.is_some() {
                    multiples.push(current.unwrap());
                } else {
                    break 'find;
                }
                i += T::one();
            }
            multiples.sort();
            multiples.dedup();
            multiples
        }

        /// Multiples
        ///
        /// Vector of all unique multiples of the input factors that are strictly below the max value
        #[allow(dead_code)]
        pub fn multiples<T>(factors: Vec<T>, max: T) -> Vec<T>
        where
            T: $t,
        {
            let mut multiples = vec![];
            for factor in factors {
                let mut i = T::one();
                'find: while i <= (max / factor) {
                    let current = factor.checked_mul(&i);
                    if current.is_some() {
                        if current.unwrap() < max {
                            multiples.push(current.unwrap());
                        } else {
                            break 'find;
                        }
                    } else {
                        break 'find;
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
        /// Sum all unique multiples of the input factors that are below the max value
        #[allow(dead_code)]
        pub fn sum_of_multiples<T>(factors: Vec<T>, max: T) -> T
        where
            T: $t,
        {
            multiples(factors, max)
                .into_iter()
                .fold(T::zero(), |acc, x| acc + x)
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

    #[test]
    fn test_multiples_of() {
        assert_eq!(
            multiples_of(300000000),
            vec![300000000, 600000000, 900000000, 1200000000, 1500000000, 1800000000, 2100000000]
        );
    }
}
