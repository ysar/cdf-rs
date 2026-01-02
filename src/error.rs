use std::{fmt::Display, io};

#[derive(Debug)]
pub enum CdfError {
    Decode(DecodeError),
    Encode(String),
    Io(io::Error),
    Other(String),
}

impl From<DecodeError> for CdfError {
    fn from(value: DecodeError) -> Self {
        CdfError::Decode(value)
    }
}

impl From<io::Error> for CdfError {
    fn from(value: io::Error) -> Self {
        CdfError::Io(value)
    }
}

impl Display for CdfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CdfError::Decode(err) => err.fmt(f),
            CdfError::Encode(_) => write!(f, "encoding error."),
            CdfError::Io(err) => err.fmt(f),
            CdfError::Other(err) => write!(f, "{err}"),
        }
    }
}

#[derive(Debug)]
pub struct DecodeError(pub String);

impl From<io::Error> for DecodeError {
    fn from(value: io::Error) -> Self {
        DecodeError(value.to_string())
    }
}
impl Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
