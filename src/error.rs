use std::io;
use thiserror::Error;

/// The top-level error to deal with module level tasks.
#[derive(Error, Debug)]
pub enum CdfError {
    /// Decoding errors
    #[error("{0:}")]
    Decode(#[from] DecodeError),

    /// Encoding errors
    #[error("{0:}")]
    Encode(#[from] EncodeError),

    /// Any IO error.
    #[error("{0:}")]
    IoError(#[from] io::Error),

    /// Any other error
    #[error("{0:}")]
    Other(String),
}

/// Decoding errors
#[derive(Error, Debug)]
pub enum DecodeError {
    /// IO error resulting from decoding.
    #[error("{0:}")]
    IoError(#[from] io::Error),

    /// If the CDF magic number is invalid.
    #[error("Invalid magic number - {0:x}")]
    InvalidMagicNumber(u32),

    /// Any other error related to decoding.
    #[error("{0:}")]
    Other(String),
}

/// Encoding errors.
#[derive(Error, Debug)]
pub enum EncodeError {
    /// IO error related to encoding.
    #[error("{0:}")]
    IoError(#[from] io::Error),

    /// Any other error resulting from encoding.
    #[error("{0:}")]
    Other(String),
}
