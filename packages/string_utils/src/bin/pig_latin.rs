use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Convert this phrase to pig latin!
    #[arg(required = true)]
    phrase: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let phrase = cli.phrase.join(" ");

    println!("{}", string_utils::phrase_to_pig_latin(&phrase));
}
