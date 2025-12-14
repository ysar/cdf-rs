use std::io;

use crate::error::{CdfError, DecodeError};
use crate::repr::{CdfEncoding, Endian};

/// Trait for decoding a CDF result from a reader.
pub trait Decodable {
    /// The value that is returned after decoding.
    type Value;

    /// Decode a value from the input that implements `io::Read`.
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>;
}

/// Struct containing the reader and decoding configurations.
pub struct Decoder<R: io::Read> {
    /// A reader is some object that implements the [io::Read] trait.
    pub reader: R,
    /// The endianness corresponding to this decoder.
    pub encoding: Endian,
}

impl<R: io::Read> Decoder<R> {
    /// Create a new decoder based on some reader than implements [io::Read] and a CDF encoding.
    pub fn new(reader: R, cdf_encoding: CdfEncoding) -> Result<Self, CdfError> {
        Ok(Decoder {
            reader,
            encoding: cdf_encoding.get_endian()?,
        })
    }
}
