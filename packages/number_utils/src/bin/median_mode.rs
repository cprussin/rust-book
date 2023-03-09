use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Calculate the median and mode of this list of numbers
    #[arg(required = true)]
    numbers: Vec<i32>,
}

fn main() {
    let mut cli = Cli::parse();

    println!("Median: {}", number_utils::median(&mut cli.numbers));
    println!("Mode: {}", number_utils::mode(&cli.numbers));
}
