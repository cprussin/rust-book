use std::env::args;

mod christmas;
mod fibonacci;
mod guess_the_number;
mod rectangle;
mod temperature;
mod util;

fn show_help() {
    println!("Please tell me what to run.  Options:");
    println!("  - christmas: sing a christmas carol");
    println!("  - fibonacci <n>: print the value of the <n>th Fibonacci number");
    println!("  - guess: guess the number!");
    println!("  - rectangle <width> <height>: calculate the area of a rectangle given the width and height");
    println!("  - temperature <t>: convert Fahrenheit value <t> to Celsius");
}

fn main() {
    match args().nth(1).as_deref() {
        Some("christmas") => christmas::main(),
        Some("fibonacci") => fibonacci::main(),
        Some("guess") => guess_the_number::main(),
        Some("rectangle") => rectangle::main(),
        Some("temperature") => temperature::main(),
        _ => show_help(),
    }
}
