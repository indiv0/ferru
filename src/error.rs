use std::fmt::{self, Formatter};
use std::io::{self, ErrorKind};
use std::path::{self, Path};
use std::string;

use parser;

/// An enum of all error kinds.
#[derive(Debug)]
pub enum Error {
    /// An error for when a type cannot be parsed to a string because it
    /// contains invalid UTF-8.
    InvalidUtf8,
    /// An IO error was encountered.
    IoError(io::Error),
    /// An error which occurs when a path which is expected to contain a file
    /// name as the final component instead terminates in `.`, `..`, or solely
    /// of a root of prefix.
    MissingFileName,
    /// An error for when the template field is expected but not specified in a
    /// document header.
    MissingTemplateField,
    /// Wraps errors emitted by methods when attempting to parse a document.
    ParserError(parser::Error),
    /// Wraps errors emitted by the `Path::strip_prefix` method.
    StripPrefixError(path::StripPrefixError),
    /// An error which occurs when a specified template cannot be found.
    TemplateNotFound,
}

impl Error {
    /// Create an error for a non-directory path which was expected to be as
    /// directory.
    pub fn path_is_not_a_directory<P>(path: &P) -> Self
        where P: AsRef<Path> + fmt::Debug,
    {
        Error::IoError(io::Error::new(ErrorKind::InvalidInput, format!(
                    "specified path is not a directory: {}",
                    path.as_ref().display(),
                )))
    }

    /// Create an error for a path which unexpectedly points at a non-existing
    /// entity.
    pub fn path_not_found<P>(path: &P) -> Self
        where P: AsRef<Path> + fmt::Debug,
    {
        Error::IoError(io::Error::new(ErrorKind::NotFound, format!(
                    "specified path not found: {}",
                    path.as_ref().display(),
                )))
    }
}

/// Application generic result type.
pub type Result<T> = ::std::result::Result<T, self::Error>;

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

impl From<parser::Error> for Error {
    fn from(error: parser::Error) -> Error {
        Error::ParserError(error)
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(_error: string::FromUtf8Error) -> Error {
        Error::InvalidUtf8
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(error: path::StripPrefixError) -> Error {
        Error::StripPrefixError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match *self {
            Error::InvalidUtf8 => "a string is not valid UTF-8".fmt(formatter),
            Error::IoError(ref e) => e.fmt(formatter),
            Error::MissingFileName => "a path is missing a file name".fmt(formatter),
            Error::MissingTemplateField => "missing template field in header".fmt(formatter),
            Error::ParserError(ref e) => e.fmt(formatter),
            Error::StripPrefixError(ref e) => e.fmt(formatter),
            Error::TemplateNotFound => "specified template could not be found".fmt(formatter),
        }
    }
}

// Implement `PartialEq` manually, since `std::io::Error` does not implement it.
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        use self::Error::{
            InvalidUtf8,
            IoError,
            MissingFileName,
            MissingTemplateField,
            ParserError,
            StripPrefixError,
            TemplateNotFound,
        };

        match (self, other) {
            (&InvalidUtf8, &InvalidUtf8)                 => true,
            (&IoError(_), &IoError(_))                   => true,
            (&MissingFileName, &MissingFileName)         => true,
            (&MissingTemplateField, &MissingFileName)    => true,
            (&ParserError(ref a), &ParserError(ref b))   => a == b,
            (&StripPrefixError(_), &StripPrefixError(_)) => true,
            (&TemplateNotFound, &TemplateNotFound)       => true,
            _ => false,
        }
    }
}
