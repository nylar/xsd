use std::str::FromStr;

use crate::shared::{BlockDefault, Final, Occurrence};
use crate::traits::TryFrom;

use roxmltree::Node;

#[derive(Debug, PartialEq)]
pub enum DefaultFixed {
    Default(String),
    Fixed(String),
}

#[derive(Debug, Default, PartialEq)]
pub struct Element {
    pub abstract_: bool,
    pub block: Option<BlockDefault>,
    pub default_fixed: Option<DefaultFixed>,
    pub final_: Option<Final>,
    pub id: Option<String>,
    pub max_occurrences: Option<Occurrence>,
    pub min_occurrences: Option<Occurrence>,
    pub name: Option<String>,
    pub nillable: bool,
    // TODO: ref
    pub substitution_group: Option<String>,
    pub type_: Option<String>,
}

impl<'a, 'd> TryFrom<Node<'a, 'd>> for Element {
    type Error = crate::errors::Error;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        let mut element = Element::default();

        if let Some(abstract_) = node.attribute("abstract") {
            element.abstract_ = abstract_ == "true";
        }

        if let Some(block) = node.attribute("block") {
            let block = BlockDefault::from_str(block)?;
            element.block = Some(block);
        }

        element.default_fixed = node
            .attribute("default")
            .and_then(|default| Some(DefaultFixed::Default(default.to_owned())));

        element.default_fixed = node
            .attribute("fixed")
            .and_then(|fixed| Some(DefaultFixed::Fixed(fixed.to_owned())));

        if let Some(final_) = node.attribute("final") {
            let final_ = Final::from_str(final_)?;
            element.final_ = Some(final_);
        }

        element.id = node.attribute("id").and_then(|id| Some(id.to_owned()));

        if let Some(max_occurrences) = node.attribute("maxOccurs") {
            let max_occurrences = Occurrence::from_str(max_occurrences)?;

            element.max_occurrences = Some(max_occurrences);
        }

        if let Some(min_occurrences) = node.attribute("minOccurs") {
            let min_occurrences = Occurrence::from_str(min_occurrences)?;

            element.min_occurrences = Some(min_occurrences);
        }

        element.name = node
            .attribute("name")
            .and_then(|name| Some(name.to_owned()));

        if let Some(nillable) = node.attribute("nillable") {
            element.nillable = nillable == "true";
        }

        element.substitution_group = node
            .attribute("substitutionGroup")
            .and_then(|sg| Some(sg.to_owned()));

        element.type_ = node.attribute("type").and_then(|ty| Some(ty.to_owned()));

        Ok(element)
    }
}
