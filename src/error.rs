use std::fmt::{self, Formatter};
use std::io::{self, ErrorKind};
use std::path::{self, Path};
use std::string;

use parser;

/// An enum of all error kinds.
#[derive(Debug)]
pub enum FerrumError {
    /// A Document is improperly formatted or missing fields.
    InvalidDocumentError(String),
    /// An error for when a type cannot be parsed to a string because it
    /// contains invalid UTF-8.
    InvalidUtf8,
    /// An IO error was encountered.
    IoError(io::Error),
    /// An error which occurs when a path which is expected to contain a file
    /// name as the final component instead terminates in `.`, `..`, or solely
    /// of a root of prefix.
    MissingFileName,
    /// Wraps errors emitted by methods when attempting to parse a document.
    ParserError(parser::Error),
    /// Wraps errors emitted by the `Path::strip_prefix` method.
    StripPrefixError(path::StripPrefixError),
}

impl FerrumError {
    /// Create an error for a missing template.
    pub fn missing_template() -> Self {
        FerrumError::InvalidDocumentError("template not found".to_owned())
    }

    /// Create an error for a missing template-specifying field in the header.
    pub fn missing_template_field() -> Self {
        FerrumError::InvalidDocumentError("missing template field in header".to_owned())
    }

    /// Create an error for a non-directory path which was expected to be as
    /// directory.
    pub fn path_is_not_a_directory<P>(path: &P) -> Self
        where P: AsRef<Path> + fmt::Debug,
    {
        FerrumError::IoError(io::Error::new(ErrorKind::InvalidInput, format!(
                    "specified path is not a directory: {}",
                    path.as_ref().display(),
                )))
    }

    /// Create an error for a path which unexpectedly points at a non-existing
    /// entity.
    pub fn path_not_found<P>(path: &P) -> Self
        where P: AsRef<Path> + fmt::Debug,
    {
        FerrumError::IoError(io::Error::new(ErrorKind::NotFound, format!(
                    "specified path not found: {}",
                    path.as_ref().display(),
                )))
    }
}

/// Application generic result type.
pub type FerrumResult<T> = ::std::result::Result<T, self::FerrumError>;

impl From<io::Error> for FerrumError {
    fn from(error: io::Error) -> FerrumError {
        FerrumError::IoError(error)
    }
}

impl From<parser::Error> for FerrumError {
    fn from(error: parser::Error) -> FerrumError {
        FerrumError::ParserError(error)
    }
}

impl From<string::FromUtf8Error> for FerrumError {
    fn from(_error: string::FromUtf8Error) -> FerrumError {
        FerrumError::InvalidUtf8
    }
}

impl From<path::StripPrefixError> for FerrumError {
    fn from(error: path::StripPrefixError) -> FerrumError {
        FerrumError::StripPrefixError(error)
    }
}

impl fmt::Display for FerrumError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            FerrumError::InvalidDocumentError(ref s) => s.fmt(formatter),
            FerrumError::InvalidUtf8 => "a string is not valid UTF-8".fmt(formatter),
            FerrumError::IoError(ref e) => e.fmt(formatter),
            FerrumError::MissingFileName => "a path is missing a file name".fmt(formatter),
            FerrumError::ParserError(ref e) => e.fmt(formatter),
            FerrumError::StripPrefixError(ref e) => e.fmt(formatter),
        }
    }
}

// Implement `PartialEq` manually, since `std::io::Error` does not implement it.
impl PartialEq<FerrumError> for FerrumError {
    fn eq(&self, other: &FerrumError) -> bool {
        use self::FerrumError::{
            InvalidDocumentError,
            InvalidUtf8,
            IoError,
            MissingFileName,
            ParserError,
            StripPrefixError,
        };

        match (self, other) {
            (&InvalidDocumentError(ref a), &InvalidDocumentError(ref b)) => a == b,
            (&InvalidUtf8, &InvalidUtf8)                 => true,
            (&IoError(_), &IoError(_))                   => true,
            (&MissingFileName, &MissingFileName)         => true,
            (&ParserError(ref a), &ParserError(ref b))   => a == b,
            (&StripPrefixError(_), &StripPrefixError(_)) => true,
            _ => false,
        }
    }
}
