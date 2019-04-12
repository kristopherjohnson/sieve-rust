use sieve::sieve_iter;

use std::env;
use std::process;

const DEFAULT_MAX: usize = 1000;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    // Optional command-line argument is maximum value.
    let max = if args.len() == 2 {
        args[1].parse().unwrap()
    } else if args.len() == 1 {
        DEFAULT_MAX
    } else {
        eprintln!("usage: {} [MAX]", args[0]);
        eprintln!("       Default maximum is {}.", DEFAULT_MAX);
        process::exit(-1);
    };

    let mut count = 0;
    for prime in sieve_iter(max) {
        print!("{} ", prime);
        count += 1;
    }
    println!();
    println!("Count of primes 2-{}: {}", max, count);
}
