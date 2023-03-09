const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn str_when(condition: bool, string: &str) -> &str {
    if condition {
        string
    } else {
        ""
    }
}

pub fn phrase_to_pig_latin(string: &str) -> String {
    string
        .split(' ')
        .filter(|str| !str.eq(&""))
        .map(word_to_pig_latin)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn word_to_pig_latin(word: &str) -> String {
    let first_letter = first_char(&first_char(word).to_lowercase().collect::<String>());
    if VOWELS.contains(&first_letter) {
        format!("{}-hay", word)
    } else {
        format!(
            "{}-{}ay",
            word.chars().skip(1).collect::<String>(),
            first_letter
        )
    }
}

fn first_char(string: &str) -> char {
    string.chars().next().unwrap()
}
