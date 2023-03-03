use clap::{Parser, Subcommand};
use rust_book::{rectangle::Rectangle, string_utils};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Calculates the area of a rectangle
    Area {
        /// The width of the rectangle
        width: f64,
        /// The height of the rectangle
        height: f64,
    },

    /// Tells you whether rect2 can fit inside rect1
    CanHold {
        /// The width of the first rectangle
        rect1_width: f64,
        /// The height of the first rectangle
        rect1_height: f64,
        /// The width of the second rectangle
        rect2_width: f64,
        /// The height of the second rectangle
        rect2_height: f64,
    },

    /// Calculate and display a square of a given size
    Square {
        /// The size of one side of the square
        size: f64,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Area { width, height } => area(*width, *height),
        Commands::CanHold {
            rect1_width,
            rect1_height,
            rect2_width,
            rect2_height,
        } => can_hold(*rect1_width, *rect1_height, *rect2_width, *rect2_height),
        Commands::Square { size } => square(*size),
    }
}

fn area(width: f64, height: f64) {
    let rectangle = Rectangle { width, height };
    println!("The area of {rectangle} is {}", rectangle.area());
}

fn can_hold(rect1_width: f64, rect1_height: f64, rect2_width: f64, rect2_height: f64) {
    let rectangle1 = Rectangle {
        width: rect1_width,
        height: rect1_height,
    };
    let rectangle2 = Rectangle {
        width: rect2_width,
        height: rect2_height,
    };
    println!(
        "{rectangle2} can{} fit within {rectangle1}",
        string_utils::str_when(!rectangle1.can_hold(&rectangle2), " not")
    )
}

fn square(size: f64) {
    println!("{}", Rectangle::<f64>::square(size))
}
