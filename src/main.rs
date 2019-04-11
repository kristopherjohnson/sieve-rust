use sieve::sieve_iter;

use std::env;
use std::process;

fn main() {
    let mut max = 100;

    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        max = args[1].parse().unwrap();
    } else if args.len() != 1 {
        eprintln!("usage: {} [MAX]", args[0]);
        eprintln!("       Default maximum is {}.", max);
        process::exit(-1);
    }

    let mut count = 0;
    for prime in sieve_iter(max) {
        print!("{} ", prime);
        count += 1;
    }
    println!();
    println!("Count of primes 2-{}: {}", max, count);
}
