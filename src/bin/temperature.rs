use rust_book::temperature::Temperature;
use std::env;

pub fn main() {
    match env::args()
        .nth(1)
        .and_then(|s| s.parse::<f64>().ok())
        .map(Temperature::Fahrenheit)
    {
        Some(f) => println!("{f} is {}", Temperature::to_celsius(&f)),
        None => println!("You must pass a temperature in Fahrenheit!"),
    }
}
