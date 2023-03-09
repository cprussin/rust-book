use std::fmt::{Display, Error, Formatter};

pub mod cli;

#[derive(Debug, PartialEq)]
pub enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}

impl Temperature {
    pub fn to_fahrenheit(&self) -> Self {
        match self {
            Self::Fahrenheit(t) => Self::Fahrenheit(*t),
            Self::Celsius(t) => Self::Fahrenheit((t * (9.0 / 5.0)) + 32.0),
        }
    }

    pub fn to_celsius(&self) -> Self {
        match self {
            Self::Celsius(t) => Self::Celsius(*t),
            Self::Fahrenheit(t) => Self::Celsius((t - 32.0) * (5.0 / 9.0)),
        }
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Celsius(t) => write!(f, "{}°C", t),
            Self::Fahrenheit(t) => write!(f, "{}°F", t),
        }
    }
}

#[cfg(test)]
mod tests {
    mod to_fahrenheit {
        use super::super::*;

        #[test]
        fn it_converts_0_celsius_to_fahrenheit() {
            assert_eq!(
                Temperature::Celsius(0.0).to_fahrenheit(),
                Temperature::Fahrenheit(32.0)
            );
        }

        #[test]
        fn it_converts_100_celsius_to_fahrenheit() {
            assert_eq!(
                Temperature::Celsius(100.0).to_fahrenheit(),
                Temperature::Fahrenheit(212.0)
            );
        }
    }

    mod to_celsius {
        use super::super::*;

        #[test]
        fn it_converts_32_fahrenheit_to_celsius() {
            assert_eq!(
                Temperature::Fahrenheit(32.0).to_celsius(),
                Temperature::Celsius(0.0)
            );
        }

        #[test]
        fn it_converts_212_fahrenheit_to_celsius() {
            assert_eq!(
                Temperature::Fahrenheit(212.0).to_celsius(),
                Temperature::Celsius(100.0)
            );
        }
    }

    mod fmt {
        use super::super::*;

        #[test]
        fn it_displays_fahrenheit() {
            assert_eq!(format!("{}", Temperature::Fahrenheit(50.0)), "50°F");
        }

        #[test]
        fn it_displays_celsius() {
            assert_eq!(format!("{}", Temperature::Celsius(50.0)), "50°C");
        }
    }
}
