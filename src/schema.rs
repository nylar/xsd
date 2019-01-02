use std::collections::HashMap;
use std::default::Default;
use std::str::FromStr;

use crate::shared::{BlockDefault, FinalDefault, FormDefault};
use crate::traits::TryFrom;

use roxmltree::Node;

#[derive(Debug, Default, PartialEq)]
pub struct Schema {
    pub attribute_form_default: Option<FormDefault>,
    pub block_default: Option<BlockDefault>,
    pub element_form_default: Option<FormDefault>,
    pub final_default: Option<FinalDefault>,
    pub id: Option<String>,
    pub namespaces: HashMap<String, String>,
    pub target_namespace: Option<String>,
    pub version: Option<String>,
    pub xml_lang: Option<String>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Schema {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut schema = Schema {
            id: node.attribute("id").and_then(|i| Some(i.to_owned())),
            namespaces: node
                .namespaces()
                .iter()
                .map(|n| match n.name() {
                    Some(name) => (name.to_owned(), n.uri().to_owned()),
                    None => ("default".to_owned(), n.uri().to_owned()),
                }).collect(),
            target_namespace: node
                .attribute("targetNamespace")
                .and_then(|t| Some(t.to_owned())),
            version: node.attribute("version").and_then(|v| Some(v.to_owned())),
            xml_lang: node.attribute("xml:lang").and_then(|x| Some(x.to_owned())),
            ..Default::default()
        };

        if let Some(attribute_form_default) = node.attribute("attributeFormDefault") {
            let attribute_form_default = FormDefault::from_str(attribute_form_default)?;

            schema.attribute_form_default = Some(attribute_form_default);
        }

        if let Some(block_default) = node.attribute("blockDefault") {
            let block_default = BlockDefault::from_str(block_default)?;

            schema.block_default = Some(block_default);
        }

        if let Some(element_form_default) = node.attribute("elementFormDefault") {
            let element_form_default = FormDefault::from_str(element_form_default)?;

            schema.element_form_default = Some(element_form_default);
        }

        if let Some(final_default) = node.attribute("finalDefault") {
            let final_default = FinalDefault::from_str(final_default)?;

            schema.final_default = Some(final_default);
        }

        Ok(schema)
    }
}
