use crate::traits::TryFrom;

use roxmltree::Node;

#[derive(Debug, PartialEq)]
pub struct Import {
    pub id: Option<String>,
    pub namespace: Option<String>,
    pub schema_location: Option<String>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Import {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        Ok(Import {
            id: node.attribute("id").and_then(|i| Some(i.to_owned())),
            namespace: node.attribute("namespace").and_then(|n| Some(n.to_owned())),
            schema_location: node
                .attribute("schemaLocation")
                .and_then(|s| Some(s.to_owned())),
        })
    }
}
