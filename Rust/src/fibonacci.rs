//! Fibonacci Sequence Functions
// Author: Christopher Negrich
// Github: cnegrich
//
// A variety of functions for computing information about the Fibonacci sequence.

/// Fibonacci
///
/// Return the nth digit of the Fibonacci sequence (indexing starts at 0)
fn fibonacci(steps: usize) -> usize {
    match steps {
        0 => 0,
        1 => 1,
        _ => fibonacci(steps - 1) + fibonacci(steps - 2),
    }
}

/// Max Fibonacci
///
/// Return the largest member of the Fibonacci sequence below max
fn max_fibonacci(max: usize) -> usize {
    let mut current = 0;
    let mut previous = 0;
    let mut n = 0;
    while current < max {
        previous = current;
        current = fibonacci(n);
        n += 1;
    }
    previous
}

/// Fibonacci sequence
///
/// Return the first (n + 1) digits of the Fibonacci sequence
/// Restated, return the digits of the Fibonacci sequence up to the nth member (indexing starts at 0)
fn fibonacci_sequence(steps: usize) -> Vec<usize> {
    (0..=steps).map(|i| fibonacci(i)).collect::<Vec<usize>>()
}

/// Max Fibonacci sequence
///
/// Return the members of the Fibonacci sequence less than or equal to max
fn max_fibonacci_sequence(max: usize) -> Vec<usize> {
    let mut sequence = vec![];
    let mut current = 0;
    let mut previous = 0;
    let mut n = 0;
    while current <= max {
        if n >= 2 {
            sequence.push(previous.clone());
        }
        previous = current;
        current = fibonacci(n);
        n += 1;
    }
    sequence.push(previous);
    sequence
}

/// Even Fibonacci sequence
///
/// Return the even digits of the Fibonacci sequence up to the nth member (indexing starts at 0)
fn even_fibonacci_sequence(steps: usize) -> Vec<usize> {
    (0..=steps)
        .map(|i| fibonacci(i))
        .filter(|n| n % 2 == 0)
        .collect::<Vec<usize>>()
}

/// Odd Fibonacci sequence
///
/// Return the odd digits of the Fibonacci sequence up to the nth member (indexing starts at 0)
fn odd_fibonacci_sequence(steps: usize) -> Vec<usize> {
    (0..=steps)
        .map(|i| fibonacci(i))
        .filter(|n| n % 2 != 0)
        .collect::<Vec<usize>>()
}

/// Even Fibonacci sequence up to max
///
/// Return the even numbers of the Fibonacci sequence whose values are less than or equal max
fn max_even_fibonacci_sequence(max: usize) -> Vec<usize> {
    let mut sequence = vec![];
    let mut current = 0;
    let mut previous = 0;
    let mut n = 0;
    while current <= max {
        if previous % 2 == 0 && n >= 2 {
            sequence.push(previous.clone());
        }
        previous = current;
        current = fibonacci(n);
        n += 1;
    }
    if previous % 2 == 0 {
        sequence.push(previous);
    }
    sequence
}

/// Odd Fibonacci sequence up to max
///
/// Return the odd numbers of the Fibonacci sequence whose values are less than or equal max
fn max_odd_fibonacci_sequence(max: usize) -> Vec<usize> {
    let mut sequence = vec![];
    let mut current = 0;
    let mut previous = 0;
    let mut n = 0;
    while current <= max {
        if previous % 2 != 0 && n >= 2 {
            sequence.push(previous.clone());
        }
        previous = current;
        current = fibonacci(n);
        n += 1;
    }
    if previous % 2 != 0 {
        sequence.push(previous);
    }
    sequence
}

/// Sum n Fibonacci sequence
///
/// Return the sum of the first (n + 1) digits of the Fibonacci sequence
fn sum_n_fibonacci(steps: usize) -> usize {
    (0..=steps).map(|i| fibonacci(i)).sum::<usize>()
}

/// Sum n even Fibonacci sequence
///
/// Return the sum of the even digits of the Fibonacci sequence up to the nth member (indexing starts at 0)
fn sum_n_even_fibonacci(steps: usize) -> usize {
    (0..=steps)
        .map(|i| fibonacci(i))
        .filter(|n| n % 2 == 0)
        .sum::<usize>()
}

/// Sum n odd Fibonacci sequence
///
/// Return the sum of the odd digits of the Fibonacci sequence up to the nth member (indexing starts at 0)
fn sum_n_odd_fibonacci(steps: usize) -> usize {
    (0..=steps)
        .map(|i| fibonacci(i))
        .filter(|n| n % 2 != 0)
        .sum::<usize>()
}

/// Fibonacci sequence sum
///
/// Return the sum of the numbers of the Fibonacci sequence whose values are less than or equal max
fn sum_max_fibonacci(max: usize) -> usize {
    max_fibonacci_sequence(max).into_iter().sum()
}

/// Even Fibonacci sequence sum
///
/// Return the sum of the even numbers of the Fibonacci sequence whose values are less than or equal max
fn sum_max_even_fibonacci(max: usize) -> usize {
    max_even_fibonacci_sequence(max).into_iter().sum()
}

/// Odd Fibonacci sequence sum
///
/// Return the sum of the odd numbers of the Fibonacci sequence whose values are less than or equal max
fn sum_max_odd_fibonacci(max: usize) -> usize {
    max_odd_fibonacci_sequence(max).into_iter().sum()
}

// Unit Tests ------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 1);
        assert_eq!(fibonacci(3), 2);
        assert_eq!(fibonacci(4), 3);
        assert_eq!(fibonacci(5), 5);
        assert_eq!(fibonacci(6), 8);
        assert_eq!(fibonacci(7), 13);
        assert_eq!(fibonacci(8), 21);
    }

    #[test]
    fn test_max_fibonacci() {
        assert_eq!(max_fibonacci(100), 89);
    }

    #[test]
    fn test_fibonacci_sqeuence() {
        assert_eq!(
            fibonacci_sequence(12),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
        );
    }

    #[test]
    fn test_max_fibonacci_sqeuence() {
        assert_eq!(
            max_fibonacci_sequence(144),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144]
        );
    }

    #[test]
    fn test_even_fibonacci_sqeuence() {
        assert_eq!(even_fibonacci_sequence(12), vec![0, 2, 8, 34, 144]);
    }

    #[test]
    fn test_odd_fibonacci_sqeuence() {
        assert_eq!(odd_fibonacci_sequence(12), vec![1, 1, 3, 5, 13, 21, 55, 89]);
    }

    #[test]
    fn test_max_even_fibonacci_sqeuence() {
        assert_eq!(max_even_fibonacci_sequence(144), vec![0, 2, 8, 34, 144]);
    }

    #[test]
    fn test_max_odd_fibonacci_sqeuence() {
        assert_eq!(
            max_odd_fibonacci_sequence(144),
            vec![1, 1, 3, 5, 13, 21, 55, 89]
        );
    }

    #[test]
    fn test_sum_n_fibonacci() {
        assert_eq!(sum_n_fibonacci(4), 7);
    }

    #[test]
    fn test_sum_n_even_fibonacci() {
        assert_eq!(sum_n_even_fibonacci(4), 2);
    }

    #[test]
    fn test_sum_n_odd_fibonacci() {
        assert_eq!(sum_n_odd_fibonacci(4), 5);
    }

    #[test]
    fn test_sum_max_fibonacci() {
        assert_eq!(sum_max_fibonacci(3), 7);
    }

    #[test]
    fn test_sum_max_even_fibonacci() {
        assert_eq!(sum_max_even_fibonacci(3), 2);
    }

    #[test]
    fn test_sum_max_odd_fibonacci() {
        assert_eq!(sum_max_odd_fibonacci(3), 5);
    }
}
