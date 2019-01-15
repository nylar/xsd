use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    UnhandledTag { parent: String, tag: String },
    InvalidFinal,
    InvalidFinalDefault,
    InvalidBlockDefault,
    InvalidFormDefault,
    InvalidUse,
    InvalidRootFolder,
    ParseInt(std::num::ParseIntError),
    IO(std::io::Error),
    ParseXml(roxmltree::Error),
    EmptyXml,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseInt(ref e) => e.fmt(f),
            Error::IO(ref e) => e.fmt(f),
            Error::ParseXml(ref e) => e.fmt(f),
            Error::UnhandledTag {
                ref parent,
                ref tag,
            } => write!(
                f,
                "{}: {} in {}",
                error::Error::description(self),
                tag,
                parent
            ),
            _ => write!(f, "{}", error::Error::description(self)),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnhandledTag { .. } => "Unhandled tag",
            Error::InvalidFinal => "Invalid final",
            Error::InvalidFinalDefault => "Invalid final default",
            Error::InvalidBlockDefault => "Invalid block default",
            Error::InvalidFormDefault => "Invalid form default",
            Error::InvalidUse => "Invalid use value",
            Error::InvalidRootFolder => "Invalid root folder",
            Error::ParseInt(ref e) => e.description(),
            Error::IO(ref e) => e.description(),
            Error::ParseXml(ref e) => e.description(),
            Error::EmptyXml => "Empty XML document",
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::ParseInt(ref e) => Some(e),
            Error::IO(ref e) => Some(e),
            Error::ParseXml(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<roxmltree::Error> for Error {
    fn from(err: roxmltree::Error) -> Error {
        Error::ParseXml(err)
    }
}
