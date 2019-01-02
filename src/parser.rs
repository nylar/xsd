use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use crate::complex_type::ComplexType;
use crate::element::Element;
use crate::errors::Error;
use crate::import::Import;
use crate::include::Include;
use crate::schema::Schema;
use crate::simple_type::SimpleType;
use crate::traits::TryFrom;
use roxmltree::{Document, Node};

const SCHEMA: &'static str = "schema";
const IMPORT: &'static str = "import";
const INCLUDE: &'static str = "include";
const ELEMENT: &'static str = "element";
const SIMPLE_TYPE: &'static str = "simpleType";
const COMPLEX_TYPE: &'static str = "complexType";

#[derive(Debug, PartialEq)]
pub enum Elements {
    Schema(Schema),
    Import(Import),
    Include(Include),
    Element(Element),
    SimpleType(SimpleType),
    ComplexType(ComplexType),
}

#[derive(Debug)]
pub struct Parser {
    pub elements: Vec<Elements>,
    root_folder: PathBuf,
}

impl Parser {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Parser, Error> {
        let root_folder = match file_path.as_ref().parent() {
            Some(rf) => rf,
            None => return Err(Error::InvalidRootFolder),
        };

        let contents = Parser::read_file(&file_path)?;

        let doc = Document::parse(&contents)?;

        let mut parser = Parser {
            elements: Vec::new(),
            root_folder: root_folder.to_path_buf(),
        };

        parser.parse(doc.root())?;

        Ok(parser)
    }

    pub fn elements(&self) -> &Vec<Elements> {
        &self.elements
    }

    fn read_file<P: AsRef<Path>>(file_path: P) -> Result<String, Error> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    fn parse(&mut self, parent_node: Node) -> Result<(), Error> {
        for node in parent_node.children().filter(|n| match n.node_type() {
            roxmltree::NodeType::Element | roxmltree::NodeType::Root => true,
            _ => false,
        }) {
            match node.tag_name().name() {
                SCHEMA => self
                    .elements
                    .push(Elements::Schema(Schema::try_from(node)?)),
                IMPORT => {
                    let import = Import::try_from(node)?;

                    if let Some(ref schema_location) = import.schema_location {
                        let contents =
                            Parser::read_file(self.root_folder.as_path().join(schema_location))?;

                        let doc = Document::parse(&contents)?;

                        self.parse(doc.root())?;
                    }

                    self.elements.push(Elements::Import(import));
                }
                INCLUDE => {
                    let include = Include::try_from(node)?;

                    if let Some(ref schema_location) = include.schema_location {
                        let contents =
                            Parser::read_file(self.root_folder.as_path().join(schema_location))?;

                        let doc = Document::parse(&contents)?;

                        self.parse(doc.root())?;
                    }

                    self.elements.push(Elements::Include(include));
                }
                ELEMENT => self
                    .elements
                    .push(Elements::Element(Element::try_from(node)?)),
                SIMPLE_TYPE => self
                    .elements
                    .push(Elements::SimpleType(SimpleType::try_from(node)?)),
                COMPLEX_TYPE => self
                    .elements
                    .push(Elements::ComplexType(ComplexType::try_from(node)?)),
                // unknown => {
                //     return Err(crate::errors::Error::UnhandledTag {
                //         parent: parent_node.tag_name().name().to_owned(),
                //         tag: unknown.to_owned(),
                //     })
                // }
                _ => (),
            }

            self.parse(node)?;
        }

        Ok(())
    }
}
