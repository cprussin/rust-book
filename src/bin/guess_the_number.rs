use rand::Rng;
use std::{
    cmp::Ordering,
    io::{self, Write},
    ops::RangeInclusive,
};

const RANGE: RangeInclusive<u32> = 1..=500;

fn parse_guess(guess: &str) -> Option<u32> {
    guess
        .parse()
        .ok()
        .and_then(|num| RANGE.contains(&num).then_some(num))
}

fn read_guess() -> String {
    let mut guess = String::new();
    print!("Please input your guess: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut guess).unwrap();
    guess.trim().to_string()
}

fn print_invalid_guess_help(guess: &str) {
    println!(
        "'{}' is not a valid guess, you must guess a number between {} and {}!",
        guess,
        RANGE.start(),
        RANGE.end()
    )
}

fn get_valid_guess() -> u32 {
    loop {
        let guess = read_guess();
        match parse_guess(&guess) {
            Some(num) => return num,
            None => print_invalid_guess_help(&guess),
        };
    }
}

pub fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(RANGE);

    loop {
        match get_valid_guess().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
