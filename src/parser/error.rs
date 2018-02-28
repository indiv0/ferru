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

use parser::serde_yaml;

/// A convenient alias type for results when parsing the Ferru document format.
pub type Result<T> = StdResult<T, Error>;

/// An enum of all error kinds.
#[derive(Debug)]
pub enum Error {
    /// Wraps errors emitted by methods during serde parsing.
    SerdeError(serde_yaml::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::SerdeError(ref e) => e.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SerdeError(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::SerdeError(ref e) => e.cause(),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Error {
        Error::SerdeError(e)
    }
}

// Implement `PartialEq` manually, since `serde_yaml::Error` does not implement
// it.
impl PartialEq<Error> for Error {
    fn eq(&self, other: &Error) -> bool {
        use self::Error::*;

        match (self, other) {
            (&SerdeError(_), &SerdeError(_)) => true,
        }
    }
}
