use clap::Parser;
use inquire::{validator::Validation, CustomType, CustomUserError};
use rand::Rng;
use std::{cmp::Ordering, ops::RangeInclusive};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

const RANGE: RangeInclusive<u32> = 1..=500;

fn main() {
    Cli::parse();

    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(RANGE);

    _ = CustomType::<u32>::new("Input your guess: ")
        .with_help_message(&format!(
            "Please enter a number between {} and {}",
            RANGE.start(),
            RANGE.end()
        ))
        .with_validator(validate_guess)
        .with_validator(move |guess: &u32| check_guess(guess, &secret))
        .with_formatter(&|i| format!("You win!  The number was {i}!"))
        .prompt();
}

fn validate_guess(guess: &u32) -> Result<Validation, CustomUserError> {
    if RANGE.contains(guess) {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid(
            format!(
                "{guess} is not between {} and {}",
                RANGE.start(),
                RANGE.end()
            )
            .into(),
        ))
    }
}

fn check_guess(guess: &u32, secret: &u32) -> Result<Validation, CustomUserError> {
    match guess.cmp(secret) {
        Ordering::Less => Ok(Validation::Invalid("Too small!".into())),
        Ordering::Greater => Ok(Validation::Invalid("Too big!".into())),
        Ordering::Equal => Ok(Validation::Valid),
    }
}
