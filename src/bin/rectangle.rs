use rust_book::{rectangle::Rectangle, string_utils::str_when};
use std::env::args;

pub fn main() {
    match args().nth(1).as_deref() {
        Some("area") => area(),
        Some("can_hold") => can_hold(),
        Some("square") => square(),
        _ => help(),
    }
}

fn area() {
    match Rectangle::<f64>::parse(args().skip(2)) {
        Some(rectangle) => println!("The area of {rectangle} is {}", rectangle.area()),
        None => println!("You must specify a width and a height!"),
    }
}

fn can_hold() {
    match (
        Rectangle::<f64>::parse(args().skip(2)),
        Rectangle::<f64>::parse(args().skip(4)),
    ) {
        (Some(rectangle1), Some(rectangle2)) => println!(
            "{rectangle2} can{} fit within {rectangle1}",
            str_when(!rectangle1.can_hold(&rectangle2), " not")
        ),
        _ => println!("You must specify a width, a height, and another width and height!"),
    }
}

fn square() {
    match args()
        .nth(2)
        .and_then(|s| s.parse::<f64>().ok())
        .map(Rectangle::square)
    {
        Some(rectangle) => println!("{rectangle}"),
        None => println!("You must provide a size for the square"),
    }
}

fn help() {
    println!("\
        You must select an operation.  Options:\n\
        \t- area <width> <height>: calculate the area of a rectangle given the width and height\n\
        \t- can_hold <width1> <height1> <width2> <height2>: determine if the second rectangle can fit in the first\n\
        \t- square <size>: construct and print a square with the given size\
    ");
}
