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

fn gifts(i: usize) -> String {
    match i {
        0 => DAYS[0].1.to_string(),
        1 => format!("{} and {}", DAYS[1].1, DAYS[0].1),
        _ => DAYS[0..i + 1]
            .iter()
            .enumerate()
            .map(|(i, day)| {
                if i == 0 {
                    format!("and {}", day.1)
                } else {
                    day.1.to_string()
                }
            })
            .rev()
            .collect::<Vec<String>>()
            .join(", "),
    }
}

pub fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        println!(
            "On the {} day of Christmas my true love sent to me {}",
            day.0,
            gifts(i)
        );
    }
}
