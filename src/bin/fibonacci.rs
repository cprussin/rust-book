use clap::Parser;
use ordinal::Ordinal;
use rust_book::number_utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
struct Cli {
    /// The index in the Fibonacci sequence
    #[arg(required = true)]
    index: u32,
}

pub fn main() {
    let cli = Cli::parse();

    println!(
        "The {} Fibonacci number is {}",
        Ordinal(cli.index),
        number_utils::fibonacci(cli.index)
    )
}
