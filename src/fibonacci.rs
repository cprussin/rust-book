use std::env::args;

fn fibonacci(index: u32) -> u32 {
    let mut seq = [0, 1];
    for _ in 0..index {
        seq = [seq[1], seq[0] + seq[1]];
    }
    seq[0]
}

pub fn main() {
    match args().nth(2).and_then(|s| s.parse::<u32>().ok()) {
        Some(n) => println!("The {n}th Fibonacci number is {}", fibonacci(n)),
        None => println!("You must pass an index!"),
    }
}