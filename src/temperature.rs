use std::env::args;

fn to_celsius(fahrenheit: f64) -> f64 {
    ((fahrenheit - 32.0) * 5.0) / 9.0
}

pub fn main() {
    match args().nth(2).and_then(|s| s.parse::<f64>().ok()) {
        Some(f) => println!(
            "{f} degrees Fahrenheit is {} degrees Celsius",
            to_celsius(f)
        ),
        None => println!("You must pass a temperature in Fahrenheit!"),
    }
}
