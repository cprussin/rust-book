use clap::Parser;
use rust_book::string_utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

const DAYS: [(&str, &str); 12] = [
    ("first", "a partridge in a pear tree"),
    ("second", "two turtle doves"),
    ("third", "three French hens"),
    ("fourth", "four calling birds"),
    ("fifth", "five gold rings"),
    ("sixth", "six geese a-laying"),
    ("seventh", "seven swans a-swimming"),
    ("eighth", "eight maids a-milking"),
    ("ninth", "nine ladies dancing"),
    ("tenth", "ten lords a-leaping"),
    ("eleventh", "eleven pipers piping"),
    ("twelfth", "twelve drummers drumming"),
];

fn main() {
    Cli::parse();

    print_header();
    for day in day_ordinals() {
        print_day(day);
    }
}

fn day_ordinals() -> Vec<(usize, String)> {
    DAYS.iter()
        .map(|day| day.0.to_string())
        .enumerate()
        .collect()
}

fn gifts(day_number: usize) -> Vec<(usize, String)> {
    DAYS[0..day_number + 1]
        .iter()
        .map(|day| day.1.to_string())
        .enumerate()
        .rev()
        .collect()
}

fn print_header() {
    println!();
    println!();
    println!("\t\t The Twelve Days Of Christmas");
    println!("\t\t==============================");
}

fn print_day((day_number, day_ordinal): (usize, String)) {
    println!();
    println!("On the {day_ordinal} day of Christmas my true love sent to me");
    for (gift_number, gift) in gifts(day_number) {
        println!(
            "\t{}{}{}",
            string_utils::str_when(day_number > 0 && gift_number == 0, "and "),
            gift,
            string_utils::str_when(day_number > 1 && gift_number > 0, ",")
        );
    }
}
