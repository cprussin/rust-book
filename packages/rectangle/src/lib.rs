//! # Rectangle!
//!
//! You know, like squares and stuff

use std::{
    fmt::{Display, Error, Formatter},
    ops::Mul,
};

pub mod cli;

#[derive(Debug)]
pub struct Rectangle<T> {
    pub width: T,
    pub height: T,
}

impl<T: Copy> Rectangle<T> {
    pub fn square(size: T) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl<T: Mul + Copy> Rectangle<T> {
    /// Calculates the area of the rectangle.
    ///
    /// Examples:
    ///
    /// ```
    /// let rectangle = rectangle::Rectangle { width: 50, height: 10 };
    /// assert_eq!(rectangle.area(), 500);
    /// ```
    pub fn area(&self) -> <T as Mul>::Output {
        self.width * self.height
    }
}

impl<T: PartialOrd> Rectangle<T> {
    pub fn can_hold(&self, other: &Self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_area() {
        let rectangle = Rectangle {
            width: 5,
            height: 10,
        };
        assert_eq!(rectangle.area(), 50);
    }
}
