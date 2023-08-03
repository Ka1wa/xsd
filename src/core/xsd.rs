use crate::result::Result;
use std::path::Path;
use roxmltree::Document;

#[derive(Clone, Debug)]
pub struct Xsd {

}

impl Default for Xsd {
    fn default() -> Self {
        Self {}
    }
}

impl Xsd {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let xml = std::fs::read_to_string(path.as_ref())?;
        let doc = Document::parse(&xml)?;

        Self::parse(doc)
    }

    fn parse(doc: Document) -> Result<Self> {
        doc.root();

        Ok(Xsd::default())
    }
}
