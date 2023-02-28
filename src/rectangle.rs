use std::{env::args, ops::Mul};

struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: Mul> Rectangle<T> {
    fn area(self) -> <T as Mul>::Output {
        self.width * self.height
    }
}

fn parse_rectangle() -> Option<Rectangle<f64>> {
    args()
        .skip(2)
        .take(2)
        .map(|elem| elem.parse().ok())
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .map(|opts| Rectangle {
            width: opts[0],
            height: opts[1],
        })
}

pub fn main() {
    match parse_rectangle() {
        Some(rectangle) => println!("The area is {}", rectangle.area()),
        None => println!("You must specify a width and a height!"),
    }
}
