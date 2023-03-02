use crate::util::str_when;
use std::{
    env::args,
    fmt::{Display, Error, Formatter},
    ops::Mul,
    str::FromStr,
};

#[derive(Debug)]
struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: Copy> Rectangle<T> {
    fn square(size: T) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl<T: FromStr + Copy> Rectangle<T> {
    fn parse<I: IntoIterator<Item = String>>(iter: I) -> Option<Self> {
        iter.into_iter()
            .take(2)
            .map(|elem| elem.parse().ok())
            .collect::<Option<Vec<_>>>()
            .filter(|vec| vec.len() == 2)
            .map(|opts| Rectangle {
                width: opts[0],
                height: opts[1],
            })
    }
}

impl<T: Mul + Copy> Rectangle<T> {
    fn area(&self) -> <T as Mul>::Output {
        self.width * self.height
    }
}

impl<T: PartialOrd> Rectangle<T> {
    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl<T: Display> Display for Rectangle<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "a rectangle of width {} and height {}",
            self.width, self.height
        )
    }
}

pub fn main() {
    match args().nth(2).as_deref() {
        Some("area") => area(),
        Some("can_hold") => can_hold(),
        Some("square") => square(),
        _ => println!("You must select an operation.  Options: area|can_hold|square"),
    }
}

fn area() {
    match Rectangle::<f64>::parse(args().skip(3)) {
        Some(rectangle) => println!("The area of {rectangle} is {}", rectangle.area()),
        None => println!("You must specify a width and a height!"),
    }
}

fn can_hold() {
    match (
        Rectangle::<f64>::parse(args().skip(3)),
        Rectangle::<f64>::parse(args().skip(5)),
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
        .nth(3)
        .and_then(|s| s.parse::<f64>().ok())
        .map(Rectangle::square)
    {
        Some(rectangle) => println!("{rectangle}"),
        None => println!("You must provide a size for the square"),
    }
}
