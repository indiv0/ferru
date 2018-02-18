// Copyright (c) 2016, 2018 Nikita Pekin and the ferru contributors
// See the README.md file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error::Error as StdError;
use std::fmt;
use std::result::Result as StdResult;

use parser::yaml_rust::{self, Yaml};

/// A convenient alias type for results when parsing the Ferru document format.
pub type Result<T> = StdResult<T, Error>;

/// An enum of all error kinds.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// An error encountered when a header is in an invalid format. Includes
    /// the problematic header.
    InvalidHeaderFormat(String),
    /// An error for when a key in a header cannot be parsed as a `String`.
    /// Includes the problematic key.
    InvalidHeaderKey(Yaml),
    /// An error for when a value in a header cannot be parsed as a `String`.
    /// Includes the problematic value.
    InvalidHeaderValue(Yaml),
    /// Wraps errors emitted by methods during YAML parsing.
    YamlError(yaml_rust::ScanError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::YamlError(ref e) => e.fmt(f),
            e => write!(f, "{}", e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidHeaderFormat(_) => "invalid header format",
            Error::InvalidHeaderKey(_) => "invalid header key",
            Error::InvalidHeaderValue(_) => "invalid header value",
            Error::YamlError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::YamlError(ref e) => e.cause(),
            _ => None,
        }
    }
}

impl From<yaml_rust::ScanError> for Error {
    fn from(e: yaml_rust::ScanError) -> Error {
        Error::YamlError(e)
    }
}
