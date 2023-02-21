#[macro_use]
pub mod error;
pub mod core;

use std::{fs::read_to_string, path::Path};
use crate::error::Error;

use quick_xml::events::Event;
use quick_xml::reader::NsReader;

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
        let mut xsd = Self::default();

        let mut reader = NsReader::from_str(&input);
        reader.trim_text(true);

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(element)) => {
                    let (ns, name) = reader.resolve_element(element.name());

                    match name.as_ref() {
                        b"schema" => {
                            let schema = core::Schema::from_xml(&mut reader, element).unwrap();
                            dbg!(schema);
                        },
                        _ => (),
                    }
                },
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }

        Ok(Self::default())
    }

    fn push_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }
}

impl Entry {
    pub fn from_xml(_: &Xsd) -> Result<Self, Error> {
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
        let _ = Xsd::parse_file("./tests/simple.xsd").unwrap();
    }
}
