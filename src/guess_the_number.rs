use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io::{stdin, stdout, Write},
    ops::RangeInclusive,
};

const RANGE: RangeInclusive<u32> = 1..=500;

fn validate_guess(guess: &String) -> Option<u32> {
    guess
        .parse()
        .ok()
        .and_then(|num| RANGE.contains(&num).then(|| num))
}

fn read_guess() -> String {
    let mut guess = String::new();
    print!("Please input your guess: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut guess).unwrap();
    guess.trim().to_string()
}

fn print_invalid_guess_help(guess: String) {
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
        match validate_guess(&guess) {
            Some(num) => return num,
            None => print_invalid_guess_help(guess),
        };
    }
}

pub fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(RANGE);

    loop {
        match get_valid_guess().cmp(&secret_number) {
            Less => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
