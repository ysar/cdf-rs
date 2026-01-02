use std::{fmt::Display, io, num::TryFromIntError};

/// Top-level error to handle all kinds of errors associated with this library.
#[derive(Debug)]
pub enum CdfError {
    /// Erros related to decoding / deserializing.
    Decode(String),
    /// Errors related to encoding / serializing.
    Encode(String),
    /// IO errors passed from [`std::io`]
    Io(io::Error),
    /// Other errors that do not belong in any other category.
    Other(String),
}

impl From<io::Error> for CdfError {
    fn from(value: io::Error) -> Self {
        CdfError::Io(value)
    }
}

impl From<TryFromIntError> for CdfError {
    fn from(value: TryFromIntError) -> Self {
        CdfError::Decode(value.to_string())
    }
}

impl Display for CdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CdfError::Decode(err) => write!(f, "{err}"),
            CdfError::Encode(_) => write!(f, "encoding error."),
            CdfError::Io(err) => err.fmt(f),
            CdfError::Other(err) => write!(f, "{err}"),
        }
    }
}
