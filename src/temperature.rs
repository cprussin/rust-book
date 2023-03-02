use std::fmt::{Display, Error, Formatter};

pub enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}

impl Temperature {
    pub fn to_fahrenheit(&self) -> Self {
        match self {
            Self::Fahrenheit(t) => Self::Fahrenheit(*t),
            Self::Celsius(t) => Self::Fahrenheit((t - 32.0) * (5.0 / 9.0)),
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
