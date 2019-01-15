use crate::traits::TryFrom;

use roxmltree::Node;

const ENUMERATION: &str = "enumeration";
const PATTERN: &str = "pattern";
const LENGTH: &str = "length";
const MIN_LENGTH: &str = "minLength";
const MAX_LENGTH: &str = "maxLength";
const MIN_INCLUSIVE: &str = "minInclusive";
const MAX_INCLUSIVE: &str = "maxInclusive";
const FRACTION_DIGITS: &str = "fractionDigits";
const TOTAL_DIGITS: &str = "totalDigits";

#[derive(Debug, PartialEq)]
pub enum Restrictions {
    Enumeration(String),
    Pattern(String),
    Length(String),
    MinLength(String),
    MaxLength(String),
    MinInclusive(String),
    MaxInclusive(String),
    FractionDigits(String),
    TotalDigits(String),
}

#[derive(Debug, PartialEq)]
pub struct Restriction {
    pub base: String,
    pub restrictions: Vec<Restrictions>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Restriction {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut restriction = Restriction {
            // TODO: Remove unwrap()
            base: node.attribute("base").unwrap().to_owned(),
            restrictions: Vec::new(),
        };

        for child in node
            .children()
            .filter(|n| n.node_type() == roxmltree::NodeType::Element)
        {
            let r = match child.tag_name().name() {
                ENUMERATION => {
                    Restrictions::Enumeration(child.attribute("value").unwrap().to_owned())
                }
                PATTERN => Restrictions::Pattern(child.attribute("value").unwrap().to_owned()),
                LENGTH => Restrictions::Length(child.attribute("value").unwrap().to_owned()),
                MIN_LENGTH => Restrictions::MinLength(child.attribute("value").unwrap().to_owned()),
                MAX_LENGTH => Restrictions::MaxLength(child.attribute("value").unwrap().to_owned()),
                MIN_INCLUSIVE => {
                    Restrictions::MinInclusive(child.attribute("value").unwrap().to_owned())
                }
                MAX_INCLUSIVE => {
                    Restrictions::MaxInclusive(child.attribute("value").unwrap().to_owned())
                }
                FRACTION_DIGITS => {
                    Restrictions::FractionDigits(child.attribute("value").unwrap().to_owned())
                }
                TOTAL_DIGITS => {
                    Restrictions::TotalDigits(child.attribute("value").unwrap().to_owned())
                }
                unknown => {
                    return Err(crate::errors::Error::UnhandledTag {
                        parent: node.tag_name().name().to_owned(),
                        tag: unknown.to_owned(),
                    })
                }
            };
            restriction.restrictions.push(r);
        }

        Ok(restriction)
    }
}
