#[macro_use]
pub mod error;

use std::{fs::read_to_string, path::Path};
use crate::error::{Error, ErrorKind};

#[derive(Clone, Debug)]
pub struct Xsd {
    entries: Vec<Entry>,
    namespace: String
}

impl Default for Xsd {
    fn default() -> Self {
        Xsd{
            entries: Vec::new(),
            namespace: String::from("xsd")
        }
    }
}

#[derive(Clone, Debug)]
pub enum Entry {
    Empty
}

impl Xsd {
    pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let input = read_to_string(path.as_ref()).unwrap();

        Self::parse(input)
    }

    pub fn parse(input: String) -> Result<Self, Error> {
        Ok(Self::default())
    }

    fn push_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

impl Entry {
    pub fn from_xml(xsd: &Xsd) -> Result<Self, Error> {
        let n = "";

        match n {
            &_ => Ok(Entry::Empty)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let input = Xsd::parse_file("./tests/simple.xsd").unwrap();
    }
}
