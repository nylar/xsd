use std::default::Default;

use crate::restriction::Restriction;
use crate::traits::TryFrom;

use roxmltree::Node;

const RESTRICTION: &'static str = "restriction";
const ANNOTATION: &'static str = "annotation";
const DOCUMENTATION: &'static str = "documentation";
const APPINFO: &'static str = "appinfo";

#[derive(Debug, Default, PartialEq)]
pub struct SimpleType {
    pub name: Option<String>,
    pub restriction: Option<Restriction>,
    pub annotation: Option<Annotation>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for SimpleType {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut simple_type = SimpleType::default();

        if let Some(name) = node.attribute("name") {
            simple_type.name = Some(name.to_owned());
        }

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                RESTRICTION => simple_type.restriction = Some(Restriction::try_from(child)?),
                ANNOTATION => simple_type.annotation = Some(Annotation::try_from(child)?),
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(simple_type)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Annotation {
    pub documentation: Vec<String>,
    pub app_info: Vec<String>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Annotation {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut annotation = Annotation::default();

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                DOCUMENTATION => annotation
                    .documentation
                    .push(child.text().unwrap().to_owned()),
                APPINFO => annotation.app_info.push(child.text().unwrap().to_owned()),
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(annotation)
    }
}
