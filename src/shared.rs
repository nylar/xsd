use std::default::Default;
use std::str::FromStr;

use crate::errors::Error;

#[derive(Debug, PartialEq)]
pub enum FormDefault {
    Qualified,
    Unqualified,
}

impl FromStr for FormDefault {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "qualified" => Ok(FormDefault::Qualified),
            "unqualified" => Ok(FormDefault::Unqualified),
            _ => Err(Error::InvalidFormDefault),
        }
    }
}

impl Default for FormDefault {
    fn default() -> Self {
        FormDefault::Unqualified
    }
}

#[derive(Debug, PartialEq)]
pub enum Final {
    Extension,
    Restriction,
    All,
}

impl FromStr for Final {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "extension" => Ok(Final::Extension),
            "restriction" => Ok(Final::Restriction),
            "#all" => Ok(Final::All),
            _ => Err(Error::InvalidFinal),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockDefault {
    Extension,
    Restriction,
    Substitution,
    All,
}

impl FromStr for BlockDefault {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "extension" => Ok(BlockDefault::Extension),
            "restriction" => Ok(BlockDefault::Restriction),
            "substitution" => Ok(BlockDefault::Substitution),
            "#all" => Ok(BlockDefault::All),
            _ => Err(Error::InvalidBlockDefault),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum FinalDefault {
    Extension,
    Restriction,
    List,
    Union,
    All,
}

impl FromStr for FinalDefault {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "extension" => Ok(FinalDefault::Extension),
            "restriction" => Ok(FinalDefault::Restriction),
            "list" => Ok(FinalDefault::List),
            "union" => Ok(FinalDefault::Union),
            "#all" => Ok(FinalDefault::All),
            _ => Err(Error::InvalidFinalDefault),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Occurrence {
    Limit(usize),
    Unbounded,
}

impl FromStr for Occurrence {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unbounded" => Ok(Occurrence::Unbounded),
            num => match num.parse::<usize>() {
                Ok(n) => Ok(Occurrence::Limit(n)),
                Err(e) => Err(Error::ParseInt(e)),
            },
        }
    }
}
