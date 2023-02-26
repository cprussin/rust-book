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

fn str_when(condition: bool, string: &str) -> String {
    (if condition { string } else { "" }).to_string()
}

pub fn main() {
    println!();
    println!();
    println!("\t\t The Twelve Days Of Christmas");
    println!("\t\t==============================");
    for (day_number, day_ordinal) in day_ordinals() {
        println!();
        println!("On the {day_ordinal} day of Christmas my true love sent to me");
        for (gift_number, gift) in gifts(day_number) {
            println!(
                "\t{}{}{}",
                str_when(day_number > 0 && gift_number == 0, "and "),
                gift,
                str_when(day_number > 1 && gift_number > 0, ",")
            );
        }
    }
}
