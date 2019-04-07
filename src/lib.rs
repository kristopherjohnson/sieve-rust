use std::f64;

/// Return a list of all prime numbers less than or equal to the given value.
pub fn sieve(n: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();
    let mut is_prime: Vec<bool> = vec![true; n + 1];

    let limit = (n as f64).sqrt() as usize;
    for p in 2..=n {
        if !is_prime[p] {
            continue;
        }
        primes.push(p);

        if p <= limit {
            let mut mult = p * p;
            while mult <= n {
                is_prime[mult] = false;
                mult += p;
            }
        }
    }

    primes
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
}
