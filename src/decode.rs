use std::io;

use semver::Version;

use crate::error::{CdfError, DecodeError};
use crate::repr::Endian;

/// Trait for decoding a CDF result from a reader.
pub trait Decodable {
    /// The value that is returned after decoding.
    type Value;

    /// Decode a value from the input that implements `io::Read`.
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>;
}

/// Struct containing the reader and decoding configurations.
pub struct Decoder<R: io::Read> {
    /// A reader is some object that implements the [`io::Read`] trait.
    pub reader: R,
    /// The endianness corresponding to this decoder.
    pub endianness: Endian,
    /// CDF version.  This  is necessary to include in the decoder since different versions have
    /// different formats.
    pub version: Version,
}

impl<R: io::Read> Decoder<R> {
    /// Create a new decoder based on some reader than implements [io::Read] and a CDF encoding.
    pub fn new(reader: R, endianness: Endian, version: Option<Version>) -> Result<Self, CdfError> {
        Ok(Decoder {
            reader,
            endianness,
            version: version.unwrap_or(Version::new(0, 0, 0)),
        })
    }

    /// Change or set the endianness of the decoder.
    pub fn set_endianness(&mut self, endianness: Endian) {
        self.endianness = endianness;
    }

    /// Change or set the CDF version of the decoder.
    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }
}
