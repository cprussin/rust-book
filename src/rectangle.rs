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
    pub fn parse<I: IntoIterator<Item = String>>(iter: I) -> Option<Self> {
        iter.into_iter()
            .take(2)
            .map(|elem| elem.parse().ok())
            .collect::<Option<Vec<_>>>()
            .filter(|vec| vec.len() == 2)
            .map(|opts| Self {
                width: opts[0],
                height: opts[1],
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
