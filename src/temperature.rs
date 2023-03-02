use std::fmt::{Display, Error, Formatter};

pub enum Temperature {
    Fahrenheit(f64),
    Celsius(f64),
}

impl Temperature {
    pub fn to_fahrenheit(&self) -> Self {
        match self {
            Temperature::Fahrenheit(t) => Temperature::Fahrenheit(*t),
            Temperature::Celsius(t) => Temperature::Fahrenheit((t - 32.0) * (5.0 / 9.0)),
        }
    }

    pub fn to_celsius(&self) -> Self {
        match self {
            Temperature::Celsius(t) => Temperature::Celsius(*t),
            Temperature::Fahrenheit(t) => Temperature::Celsius((t - 32.0) * (5.0 / 9.0)),
        }
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Temperature::Celsius(t) => write!(f, "{}°C", t),
            Temperature::Fahrenheit(t) => write!(f, "{}°F", t),
        }
    }
}
