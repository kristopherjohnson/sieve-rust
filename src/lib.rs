use std::f64;

/// Return a `Vec` of all prime numbers less than or equal to the given value.
///
/// # Example
///
/// ```
/// # use sieve::sieve;
/// let primes = sieve(100);
/// assert_eq!(primes.len(), 25);
/// assert_eq!(primes[0], 2);
/// assert_eq!(primes[1], 3);
/// assert_eq!(primes[2], 5);
/// // ...
/// assert_eq!(primes[22], 83);
/// assert_eq!(primes[23], 89);
/// assert_eq!(primes[24], 97);

pub fn sieve(max: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();
    let mut is_prime: Vec<bool> = vec![true; max + 1];

    let limit = (max as f64).sqrt() as usize;
    for p in 2..=max {
        if !is_prime[p] {
            continue;
        }
        primes.push(p);

        if p <= limit {
            let mut mult = p * p;
            while mult <= max {
                is_prime[mult] = false;
                mult += p;
            }
        }
    }

    primes
}

/// Return an iterator that will return prime numbers up to the given maximum.
///
/// # Example
///
/// ```
/// # use sieve::sieve_iter;
/// let mut primes = String::from("Primes:");
/// for prime in sieve_iter(20) {
///     primes = primes + &format!(" {}", prime);
/// }
/// assert_eq!(primes, "Primes: 2 3 5 7 11 13 17 19");

pub fn sieve_iter(max: usize) -> Iter {
    Iter::with_maximum(max)
}

/// Iterator for prime numbers up to some maximum value.
///
/// Returned by the `sieve_iter(max)` function.
///
/// # Example
///
/// ```
/// # use sieve::{sieve_iter, Iter};
/// let mut iter: Iter = sieve_iter(100);
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), Some(3));
/// assert_eq!(iter.next(), Some(5));

pub struct Iter {
    max: usize,
    is_prime: Vec<bool>,
    next_p: usize,
}

impl Iter {
    fn with_maximum(max: usize) -> Iter {
        Iter {
            max,
            is_prime: vec![true; max + 1],
            next_p: 2,
        }
    }
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let mut p = self.next_p;
        while p <= self.max && !self.is_prime[p] {
            p += 1;
        }
        if p > self.max {
            return None;
        }

        self.next_p = p + 1;

        let mut mult = p * p;
        while mult <= self.max {
            self.is_prime[mult] = false;
            mult += p;
        }

        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2() {
        let primes = sieve(2);
        assert_eq!(primes.len(), 1);
        assert_eq!(primes[0], 2);
    }

    #[test]
    fn test_100() {
        let primes = sieve(100);
        assert_eq!(primes.len(), 25);
        assert_eq!(primes[0], 2);
        assert_eq!(primes[24], 97);
    }

    #[test]
    fn test_1000() {
        let primes = sieve(1000);
        assert_eq!(primes.len(), 168);
        assert_eq!(primes[0], 2);
        assert_eq!(primes[167], 997);
    }

    #[test]
    fn test_iter() {
        let mut iter = sieve_iter(15);
        assert_eq!(iter.next().unwrap(), 2);
        assert_eq!(iter.next().unwrap(), 3);
        assert_eq!(iter.next().unwrap(), 5);
        assert_eq!(iter.next().unwrap(), 7);
        assert_eq!(iter.next().unwrap(), 11);
        assert_eq!(iter.next().unwrap(), 13);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iter_1000() {
        let primes: Vec<usize> = sieve_iter(1000).collect();
        assert_eq!(primes.len(), 168);
        assert_eq!(primes[0], 2);
        assert_eq!(primes[167], 997);
    }
}
