use regex::Regex;
use std::{
    fs::File,
    io::{self, Error, Read},
    path::Path,
};

pub mod cli;

pub trait Grep<E> {
    fn grep(&self, pattern: &Regex) -> Result<Vec<String>, E>;
}

impl<E> Grep<E> for String {
    fn grep(&self, pattern: &Regex) -> Result<Vec<String>, E> {
        Ok(self
            .lines()
            .filter(|line| pattern.is_match(line))
            .map(String::from)
            .collect())
    }
}

impl Grep<Error> for File {
    fn grep(&self, pattern: &Regex) -> io::Result<Vec<String>> {
        let mut contents = String::new();
        self.try_clone()?.read_to_string(&mut contents)?;
        contents.grep(pattern)
    }
}

impl Grep<Error> for Path {
    fn grep(&self, pattern: &Regex) -> io::Result<Vec<String>> {
        File::open(self)?.grep(pattern)
    }
}
