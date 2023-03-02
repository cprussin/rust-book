use std::{
    fmt::{Display, Error, Formatter},
    ops::Mul,
    str::FromStr,
};

#[derive(Debug)]
pub struct Rectangle<T> {
    width: T,
    height: T,
}

impl<T: Copy> Rectangle<T> {
    pub fn square(size: T) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl<T: FromStr + Copy> Rectangle<T> {
    pub fn parse<I: IntoIterator<Item = String>>(iterable: I) -> Option<Self> {
        let mut iter = iterable.into_iter();

        Some(Self {
            width: iter.next()?.parse().ok()?,
            height: iter.next()?.parse().ok()?,
        })
    }
}

impl<T: Mul + Copy> Rectangle<T> {
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
