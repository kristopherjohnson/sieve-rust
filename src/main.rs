use sieve::sieve;

fn main() {
    let n = 100;
    let primes = sieve(n);
    println!("Found {} primes <= {}:", primes.len(), n);
    for prime in primes {
        print!("{} ", prime);
    }
    println!();
}
