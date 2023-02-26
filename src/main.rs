use std::env::args;

mod christmas;
mod fibonacci;
mod guess_the_number;
mod temperature;

fn show_help() {
    println!("Please tell me what to run.  Options:");
    println!("  - guess: guess the number!");
    println!("  - temperature <t>: convert Fahrenheit value <t> to Celsius");
    println!("  - fibonacci <n>: print the value of the <n>th Fibonacci number");
    println!("  - christmas: sing a christmas carol");
}

fn main() {
    match args().nth(1).as_ref().map(|s| s.as_str()) {
        Some("christmas") => christmas::main(),
        Some("fibonacci") => fibonacci::main(),
        Some("guess") => guess_the_number::main(),
        Some("temperature") => temperature::main(),
        _ => show_help(),
    }
}
