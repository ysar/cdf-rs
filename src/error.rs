use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("{0:}")]
    IoError(#[from] io::Error),

    #[error("Invalid magic number - {0:x}")]
    InvalidMagicNumber(u32),

    #[error("{0:}")]
    Other(String),
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("{0:}")]
    IoError(#[from] io::Error),

    #[error("{0:}")]
    Other(String),
}
