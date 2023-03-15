use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub enum Status {
    Ok,
    BadRequest,
    NotFound,
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Self::Ok => 200,
            Self::BadRequest => 400,
            Self::NotFound => 404,
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Ok => write!(f, "OK"),
            Self::BadRequest => write!(f, "Malformed Request"),
            Self::NotFound => write!(f, "Not Found"),
        }
    }
}
