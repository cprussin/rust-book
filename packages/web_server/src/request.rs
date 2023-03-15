use crate::{Headers, Method, ParseError, Version};
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub target: String,
    pub version: Version,
    pub headers: Headers,
    pub body: Option<String>,
}

impl Request {
    /// Parse a TcpStream into a Request.
    ///
    /// TODO:
    ///   - Parse bodies
    ///   - Optimize string creations (maybe we don't need as many Strings)
    ///   - Clean up code / improve error handling
    pub fn parse(stream: &mut TcpStream) -> Result<Self, ParseError> {
        let mut lines = BufReader::new(stream).lines();
        let start_line = lines
            .next()
            .ok_or(ParseError::NoData)?
            .map_err(|_| ParseError::NoData)?;
        let mut start_line_parts = start_line.split(' ');
        Ok(Self {
            method: start_line_parts
                .next()
                .ok_or(ParseError::InvalidStartLine)?
                .parse()?,
            target: String::from(
                start_line_parts
                    .next()
                    .ok_or(ParseError::InvalidStartLine)?,
            ),
            version: start_line_parts
                .next()
                .ok_or(ParseError::InvalidStartLine)?
                .parse()?,
            headers: Headers::parse(&mut lines)?,
            body: None,
        })
    }
}
