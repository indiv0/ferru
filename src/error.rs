use std::{fmt, io, result};
use std::error::FromError;
use std::fmt::Formatter;

/// An enum of all error kinds.
#[deriving(PartialEq, Eq, Clone, Show)]
pub enum FerrumError {
    /// Failed to decode a file.
    DecodingError(String),
    /// The configuration file is improperly formatted.
    InvalidConfigError,
    /// A Document is improperly formatted.
    InvalidDocumentError(String),
    /// An IO error was encountered.
    IoError(io::IoError),
    /// A rust-mustache rendering error.
    MustacheError,
    /// Failed to parse a string with the parser.
    ParserError(String),
}

/// Application generic result type.
pub type FerrumResult<T> = ::std::result::Result<T, self::FerrumError>;

impl FromError<io::IoError> for FerrumError {
    fn from_error(e: io::IoError) -> FerrumError {
        FerrumError::IoError(e)
    }
}

impl fmt::Display for FerrumError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            FerrumError::DecodingError(ref s) => s.fmt(formatter),
            FerrumError::InvalidConfigError => "Invalid configuration".fmt(formatter),
            FerrumError::InvalidDocumentError(ref s) => s.fmt(formatter),
            FerrumError::IoError(ref e) => e.fmt(formatter),
            FerrumError::MustacheError => "Mustache error".fmt(formatter),
            FerrumError::ParserError(ref s) => s.fmt(formatter),
        }
    }
}

impl fmt::Debug for FerrumError {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), fmt::Error> {
        match *self {
            FerrumError::DecodingError(ref s) => s.fmt(formatter),
            FerrumError::InvalidConfigError => "Invalid configuration".fmt(formatter),
            FerrumError::InvalidDocumentError(ref s) => s.fmt(formatter),
            FerrumError::IoError(ref e) => e.fmt(formatter),
            FerrumError::MustacheError => "Mustache error".fmt(formatter),
            FerrumError::ParserError(ref s) => s.fmt(formatter),
        }
    }
}
