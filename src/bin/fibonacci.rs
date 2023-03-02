use ordinal::Ordinal;
use rust_book::number_utils;
use std::env;

pub fn main() {
    match env::args().nth(1).and_then(|s| s.parse::<u32>().ok()) {
        Some(n) => println!(
            "The {} Fibonacci number is {}",
            Ordinal(n),
            number_utils::fibonacci(n)
        ),
        None => println!("You must pass an index!"),
    }
}
