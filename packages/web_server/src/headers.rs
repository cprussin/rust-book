use crate::ParseError;
use std::{
    collections::{hash_map::Iter, HashMap},
    io::{BufRead, Lines},
};

#[derive(Debug)]
pub struct Headers(HashMap<String, String>);

impl Headers {
    pub fn parse<T: BufRead>(lines: &mut Lines<T>) -> Result<Self, ParseError> {
        let mut headers = Self(HashMap::new());
        for line in lines.take_while(|line| line.as_ref().map(|s| !s.is_empty()).unwrap_or(false)) {
            let line = line.map_err(|_| ParseError::InvalidHeader)?;
            let mut line_parts = line.split(": ");
            headers.0.insert(
                String::from(line_parts.next().ok_or(ParseError::InvalidHeader)?),
                String::from(line_parts.next().ok_or(ParseError::InvalidHeader)?),
            );
        }
        Ok(headers)
    }

    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    pub fn iter(&self) -> Iter<'_, String, String> {
        self.0.iter()
    }
}
