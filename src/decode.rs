use std::io;

use crate::error::CdfError;
use crate::repr::{CdfEncoding, CdfVersion};
use crate::types::{CdfInt4, CdfInt8};

/// Trait for decoding a CDF result from a reader.
pub trait Decodable {
    /// The value that is returned after decoding.
    type Value;

    /// Decode a value from the input that implements `io::Read` and `io::Seek` using Big-Endian
    /// encoding.
    /// # Errors
    /// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek;

    /// Decode a value from the input that implements `io::Read` and `io::Seek` using Little-Endian
    /// encoding.
    /// # Errors
    /// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
    fn decode_le<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek;
}

/// Struct containing the reader and decoding configurations.
pub struct Decoder<R>
where
    R: io::Read + io::Seek,
{
    /// A reader is some object that implements [`io::Read`] and [`io::Seek`].
    pub reader: R,
    /// The "encoding" of the values in the CDF. This has to be read in or specified for every
    /// CDF file and is contained in the CDR.
    pub encoding: CdfEncoding,
    /// CDF version.  This  is necessary to include in the decoder since different versions have
    /// different formats.
    pub version: CdfVersion,
}

impl<R> Decoder<R>
where
    R: io::Read + io::Seek,
{
    /// Create a new decoder based on some reader than implements [`io::Read`] and a CDF encoding.
    /// # Errors
    /// Returns a [`CdfError`] if the decoder cannot be constructed.
    pub fn new(reader: R) -> Result<Self, CdfError> {
        Ok(Decoder {
            reader,
            encoding: CdfEncoding::Unspecified,
            version: CdfVersion::new(0, 0, 0),
        })
    }

    /// Sets the version of the CDF file that this decoder is currently decoding.
    pub fn set_version(&mut self, version: CdfVersion) {
        self.version = version;
    }
}

/// CDF versions prior to 3.0 use 4-byte signed integer to store file-offsets pointing to various
/// records.  This was changed to 8-bytes after 3.0.  So, we need to do version-aware decoding.
/// Safely converts [`CdfInt4`] to [`CdfInt8`] after decoding.
/// # Errors
/// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
pub fn decode_version3_int4_int8<R>(decoder: &mut Decoder<R>) -> Result<CdfInt8, CdfError>
where
    R: io::Read + io::Seek,
{
    if decoder.version.major >= 3 {
        CdfInt8::decode_be(decoder)
    } else {
        let s: i32 = CdfInt4::decode_be(decoder)?.into();
        Ok(CdfInt8::from(i64::from(s)))
    }
}
