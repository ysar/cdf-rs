use std::io;

use crate::error::{DecodeError, EncodeError};

/// Trait for encoding to a CDF file format.
pub trait Encode<W>
where
    W: io::Write,
{
    type Input;

    /// Encode an input value to something that implements `io::Write`.
    fn encode(input: Self::Input) -> Result<W, EncodeError>;
}

/// Trait for decoding a CDF result from a reader.
pub trait Decode<R>
where
    R: io::Read,
{
    type Output;

    /// Decode a value from the input that implements `io::Read`.
    fn decode(reader: R) -> Result<Self::Output, DecodeError>;
}
