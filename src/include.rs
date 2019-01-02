use crate::traits::TryFrom;

use roxmltree::Node;

#[derive(Debug, PartialEq)]
pub struct Include {
    pub id: Option<String>,
    pub schema_location: Option<String>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Include {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Include {
            id: node.attribute("id").and_then(|id| Some(id.to_owned())),
            schema_location: node
                .attribute("schemaLocation")
                .and_then(|schema_location| Some(schema_location.to_owned())),
        })
    }
}
