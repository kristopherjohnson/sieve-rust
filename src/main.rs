use sieve::sieve_iter;

use std::env;
use std::process;

const DEFAULT_MAX: usize = 1000;

fn print_help_and_exit(program_name: &str) -> ! {
    eprintln!("usage: {} [MAX]", program_name);
    eprintln!("       Default maximum is {}.", DEFAULT_MAX);
    process::exit(-1)
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    // Optional command-line argument is maximum value.
    let max = if args.len() == 2 {
        match args[1].parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("error: invalid argument: \"{}\"", &args[1]);
                print_help_and_exit(&args[0])
            }
        }
    } else if args.len() == 1 {
        DEFAULT_MAX
    } else {
        eprintln!("error: too many arguments");
        print_help_and_exit(&args[0])
    };

    let mut count = 0;
    for prime in sieve_iter(max) {
        print!("{} ", prime);
        count += 1;
    }
    println!();
    println!("Count of primes 2-{}: {}", max, count);
}
