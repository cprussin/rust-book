use crate::ParseError;
use std::str::FromStr;

#[derive(Debug)]
pub enum Version {
    V0_9,
    V1_0,
    V1_1,
    V2,
    V3,
}

impl FromStr for Version {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/0.9" => Ok(Self::V0_9),
            "HTTP/1.0" => Ok(Self::V1_0),
            "HTTP/1.1" => Ok(Self::V1_1),
            "HTTP/2" => Ok(Self::V2),
            "HTTP/3" => Ok(Self::V3),
            _ => Err(ParseError::InvalidVersion),
        }
    }
}
