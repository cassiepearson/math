//! Multiples Functions
// Author: Christopher Negrich
// Github: cnegrich
//
// Sum functions for computing multiples, multiple sums, etc.

/// Multiples
///
/// Vector of all multiples of the input factors that are strictly below the max value
fn multiples(factors: Vec<usize>, max: usize) -> Vec<usize> {
    let mut multiples = factors
        .into_iter()
        .flat_map(|factor| {
            (1..=max / factor)
                .map(|n| factor * n)
                .filter(|&result| result < max)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<usize>>();

    multiples.sort();
    multiples.dedup();
    multiples
}

/// Sum of multiples
///
/// Sum all multiples of the input factors that are below the max value
fn sum_of_multiples(factors: Vec<usize>, max: usize) -> usize {
    multiples(factors, max).into_iter().sum()
}

// Unit Tests ------------------------------------------------------------------
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
