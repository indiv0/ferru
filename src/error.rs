// Copyright (c) 2016 Nikita Pekin and the ferrum contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::fmt;
use std::io::{self, ErrorKind};
use std::path::{self, Path};
use std::string;
use mustache;

use parser;

/// A convenient alias type for results when using Ferrum.
pub type Result<T> = ::std::result::Result<T, self::Error>;

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
    /// Wraps errors emitted by methods during mustache templating.
    MustacheError(mustache::Error),
    /// Wraps errors emitted by methods when attempting to parse a document.
    ParserError(parser::Error),
    /// Wraps errors emitted by the `Path::strip_prefix` method.
    StripPrefixError(path::StripPrefixError),
    /// An error which occurs when a specified template cannot be found.
    TemplateNotFound,
}

impl Error {
    /// Create an error for when the parent of a path cannot be obtained (e.g.
    /// if the path terminates in a root or prefix).
    pub fn missing_parent_path<P>(path: &P) -> Self
        where P: AsRef<Path> + fmt::Debug,
    {
        Error::IoError(io::Error::new(ErrorKind::InvalidInput, format!(
                    "unable to get parent of path: {}",
                    path.as_ref().display(),
                )))
    }

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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::IoError(ref e) => e.fmt(f),
            &Error::MustacheError(ref e) => e.fmt(f),
            &Error::ParserError(ref e) => e.fmt(f),
            &Error::StripPrefixError(ref e) => e.fmt(f),
            e => write!(f, "{}", e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidUtf8 => "a string is not valid UTF-8",
            Error::IoError(ref e) => e.description(),
            Error::MissingFileName => "a path is missing a file name",
            Error::MissingTemplateField => "missing template field in header",
            Error::MustacheError(ref e) => e.description(),
            Error::ParserError(ref e) => e.description(),
            Error::StripPrefixError(ref e) => e.description(),
            Error::TemplateNotFound => "specified template could not be found",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::IoError(ref e) => e.cause(),
            Error::MustacheError(ref e) => e.cause(),
            Error::ParserError(ref e) => e.cause(),
            Error::StripPrefixError(ref e) => e.cause(),
            _ => None,
        }
    }
}

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

impl From<mustache::Error> for Error {
    fn from(error: mustache::Error) -> Error {
        Error::MustacheError(error)
    }
}

impl From<path::StripPrefixError> for Error {
    fn from(error: path::StripPrefixError) -> Error {
        Error::StripPrefixError(error)
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
            MustacheError,
            ParserError,
            StripPrefixError,
            TemplateNotFound,
        };

        match (self, other) {
            (&InvalidUtf8, &InvalidUtf8) |
                (&IoError(_), &IoError(_)) |
                (&MissingFileName, &MissingFileName) |
                (&MissingTemplateField, &MissingFileName) |
                (&MustacheError(_), &MustacheError(_)) |
                (&TemplateNotFound, &TemplateNotFound) => true,
            (&ParserError(ref a), &ParserError(ref b))           => a == b,
            (&StripPrefixError(ref a), &StripPrefixError(ref b)) => a == b,
            _ => false,
        }
    }
}
