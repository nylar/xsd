use std::convert::From;
use std::default::Default;
use std::str::FromStr;

use crate::element::Element;
use crate::errors::Error;
use crate::restriction::Restriction;
use crate::shared::Occurrence;
use crate::traits::TryFrom;

use roxmltree::Node;

const SEQUENCE: &'static str = "sequence";
const ATTRIBUTE: &'static str = "attribute";
const SIMPLE_CONTENT: &'static str = "simpleContent";
const COMPLEX_CONTENT: &'static str = "complexContent";
const ELEMENT: &'static str = "element";
const ANY: &'static str = "any";
const CHOICE: &'static str = "choice";
const RESTRICTION: &'static str = "restriction";
const EXTENSION: &'static str = "extension";

#[derive(Debug, Default, PartialEq)]
pub struct ComplexType {
    pub name: Option<String>,
    pub sequence: Option<Sequence>,
    pub attribute: Option<Attribute>,
    pub simple_content: Option<SimpleContent>,
    pub choice: Option<Choice>,
    pub complex_content: Option<ComplexContent>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for ComplexType {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut complex_type = ComplexType::default();

        if let Some(name) = node.attribute("name") {
            complex_type.name = Some(name.to_owned());
        }

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                SEQUENCE => complex_type.sequence = Some(Sequence::try_from(child)?),
                ATTRIBUTE => complex_type.attribute = Some(Attribute::try_from(child)?),
                SIMPLE_CONTENT => {
                    complex_type.simple_content = Some(SimpleContent::try_from(child)?)
                }
                CHOICE => complex_type.choice = Some(Choice::try_from(child)?),
                COMPLEX_CONTENT => {
                    complex_type.complex_content = Some(ComplexContent::try_from(child)?)
                }
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(complex_type)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Sequence {
    pub min_occurrences: Option<Occurrence>,
    pub max_occurrences: Option<Occurrence>,
    pub elements: Vec<Element>,
    pub anys: Vec<Any>,
    pub choice: Option<Choice>,
    pub sequences: Vec<Sequence>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Sequence {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut sequence = Sequence::default();

        if let Some(min_occurrences) = node.attribute("minOccurs") {
            sequence.min_occurrences = Some(Occurrence::from_str(min_occurrences).unwrap());
        }

        if let Some(max_occurrences) = node.attribute("maxOccurs") {
            sequence.max_occurrences = Some(Occurrence::from_str(max_occurrences).unwrap());
        }

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                ELEMENT => sequence.elements.push(Element::try_from(child)?),
                ANY => sequence.anys.push(Any::from(child)),
                CHOICE => sequence.choice = Some(Choice::try_from(child)?),
                SEQUENCE => sequence.sequences.push(Sequence::try_from(child)?),
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(sequence)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Any {
    pub min_occurrences: Option<Occurrence>,
    pub max_occurrences: Option<Occurrence>,
    pub namespace: Option<String>,
    // TODO: processContents: Enum {strict, lax, skip}
}

impl<'a, 'd> From<Node<'a, 'd>> for Any {
    fn from(node: Node) -> Self {
        let mut any = Any::default();

        if let Some(min_occurrences) = node.attribute("minOccurs") {
            any.min_occurrences = Some(Occurrence::from_str(min_occurrences).unwrap());
        }

        if let Some(max_occurrences) = node.attribute("maxOccurs") {
            any.max_occurrences = Some(Occurrence::from_str(max_occurrences).unwrap());
        }

        if let Some(namespace) = node.attribute("namespace") {
            any.namespace = Some(namespace.to_owned());
        }

        any
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Choice {
    pub min_occurrences: Option<Occurrence>,
    pub max_occurrences: Option<Occurrence>,
    pub elements: Vec<Element>,
    pub any: Option<Any>,
    pub sequences: Vec<Sequence>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Choice {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut choice = Choice::default();

        if let Some(min_occurrences) = node.attribute("minOccurs") {
            choice.min_occurrences = Some(Occurrence::from_str(min_occurrences).unwrap());
        }

        if let Some(max_occurrences) = node.attribute("maxOccurs") {
            choice.max_occurrences = Some(Occurrence::from_str(max_occurrences).unwrap());
        }

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                ELEMENT => choice.elements.push(Element::try_from(child)?),
                ANY => choice.any = Some(Any::from(child)),
                SEQUENCE => choice.sequences.push(Sequence::try_from(child)?),
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(choice)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Attribute {
    pub name: Option<String>,
    pub usage: Usage,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Attribute {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut attribute = Attribute::default();

        if let Some(name) = node.attribute("name") {
            attribute.name = Some(name.to_owned());
        }

        if let Some(usage) = node.attribute("use") {
            attribute.usage = Usage::from_str(usage)?;
        }

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(attribute)
    }
}

#[derive(Debug, PartialEq)]
pub enum Usage {
    Optional,
    Prohibited,
    Required,
}

impl Default for Usage {
    fn default() -> Self {
        Usage::Optional
    }
}

impl FromStr for Usage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "optional" => Ok(Usage::Optional),
            "prohibited" => Ok(Usage::Prohibited),
            "required" => Ok(Usage::Required),
            _ => Err(Error::InvalidUse),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Extension {
    pub base: String,
    pub attribute: Option<Attribute>,
    pub sequence: Option<Sequence>,
    pub choice: Option<Choice>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Extension {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut extension = Extension {
            base: node.attribute("base").unwrap().to_owned(),
            attribute: None,
            sequence: None,
            choice: None,
        };

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            match child.tag_name().name() {
                ATTRIBUTE => extension.attribute = Some(Attribute::try_from(child)?),
                SEQUENCE => extension.sequence = Some(Sequence::try_from(child)?),
                CHOICE => extension.choice = Some(Choice::try_from(child)?),
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            }
        }

        Ok(extension)
    }
}

#[derive(Debug, PartialEq)]
pub struct SimpleContent {
    pub content: Content,
}

#[derive(Debug, PartialEq)]
pub enum Content {
    Restriction(Restriction),
    Extension(Extension),
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for SimpleContent {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let child = node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
            .nth(0)
            .unwrap();

        match child.tag_name().name() {
            RESTRICTION => Ok(SimpleContent {
                content: Content::Restriction(Restriction::try_from(child)?),
            }),
            EXTENSION => Ok(SimpleContent {
                content: Content::Extension(Extension::try_from(child)?),
            }),
            unknown => {
                return Err(crate::errors::Error::UnhandledTag {
                    parent: node.tag_name().name().to_owned(),
                    tag: unknown.to_owned(),
                })
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ComplexContent {
    pub content: Content,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for ComplexContent {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let child = node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
            .nth(0)
            .unwrap();

        match child.tag_name().name() {
            RESTRICTION => Ok(ComplexContent {
                content: Content::Restriction(Restriction::try_from(child)?),
            }),
            EXTENSION => Ok(ComplexContent {
                content: Content::Extension(Extension::try_from(child)?),
            }),
            unknown => {
                return Err(crate::errors::Error::UnhandledTag {
                    parent: node.tag_name().name().to_owned(),
                    tag: unknown.to_owned(),
                })
            }
        }
    }
}
