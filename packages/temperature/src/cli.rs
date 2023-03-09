use crate::Temperature;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))]
struct Cli {
    /// The temperature in degrees Fahrenheit
    #[arg(required = true)]
    fahrenheit: f64,
}

pub fn main() {
    let cli = Cli::parse();
    let f = Temperature::Fahrenheit(cli.fahrenheit);

    println!("{f} is {}", f.to_celsius());
}
