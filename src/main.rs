use sieve::sieve_iter;

use std::env;

fn main() {
    let mut max = 100;

    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        max = usize::from_str_radix(&args[1], 10).unwrap();
    }

    let mut count = 0;
    for prime in sieve_iter(max) {
        print!("{} ", prime);
        count += 1;
    }
    println!();
    println!("Count of primes 2-{}: {}", max, count);
}
