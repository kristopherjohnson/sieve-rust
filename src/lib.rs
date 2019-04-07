use std::f64;

/// Return a `Vec` of all prime numbers less than or equal to the given value.
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
pub fn sieve_iter(n: usize) -> Iter {
    Iter::with_maximum(n)
}

/// Iterator for prime numbers up to some maximum value.
pub struct Iter {
    max: usize,
    is_prime: Vec<bool>,
    next_index: usize,
}

impl Iter {
    fn with_maximum(max: usize) -> Iter {
        Iter {
            max,
            is_prime: vec![true; max + 1],
            next_index: 2,
        }
    }
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while self.next_index <= self.max && !self.is_prime[self.next_index] {
            self.next_index += 1;
        }
        if self.next_index >= self.max {
            return None;
        }

        let p = self.next_index;
        self.next_index += 1;

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
