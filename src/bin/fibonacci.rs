use ordinal::Ordinal;
use rust_book::number_utils::fibonacci;
use std::env::args;

pub fn main() {
    match args().nth(1).and_then(|s| s.parse::<u32>().ok()) {
        Some(n) => println!("The {} Fibonacci number is {}", Ordinal(n), fibonacci(n)),
        None => println!("You must pass an index!"),
    }
}
