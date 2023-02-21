use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::name::ResolveResult;
use quick_xml::reader::NsReader;

#[derive(Clone, Debug)]
pub struct Schema {
    pub namespaces: HashMap<String, String>,
}

impl Default for Schema {
    fn default() -> Self {
        Schema{
            namespaces: HashMap::new()
        }
    }
}

impl Schema {
    pub fn from_xml(
        reader: &mut NsReader<&[u8]>,
        element: BytesStart,
    ) -> Result<Schema, quick_xml::Error> {
        let mut schema = Schema::default();

        for attr_result in element.attributes() {
            let attr = attr_result?;
            let key: String = reader.decoder().decode(attr.key.as_ref())?.into_owned().into();

            if key.starts_with("xmlns:") {
                let mut split = key.split(":");
                let _ = split.next();

                if let Some(ns) = split.next() {
                    let value: String = attr.decode_and_unescape_value(reader)?.into();

                    schema.namespaces.insert(ns.into(), value);
                }
            }
        }

        Ok(schema)
    }
}
