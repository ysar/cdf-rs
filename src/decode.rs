use std::io;

use crate::error::{CdfError, DecodeError};
use crate::repr::{CdfEncoding, CdfVersion};
use crate::types::{CdfInt4, CdfInt8};

/// Trait for decoding a CDF result from a reader.
pub trait Decodable {
    /// The value that is returned after decoding.
    type Value;

    /// Decode a value from the input that implements `io::Read` and `io::Seek` using Big-Endian
    /// encoding.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>
    where
        R: io::Read + io::Seek;

    /// Decode a value from the input that implements `io::Read` and `io::Seek` using Little-Endian
    /// encoding.
    fn decode_le<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>
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
    /// Create a new decoder based on some reader than implements [io::Read] and a CDF encoding.
    pub fn new(reader: R) -> Result<Self, CdfError> {
        Ok(Decoder {
            reader,
            encoding: CdfEncoding::Unspecified,
            version: CdfVersion::new(0, 0, 0),
        })
    }

    pub fn set_version(&mut self, version: CdfVersion) {
        self.version = version;
    }
}

/// CDF versions prior to 3.0 use 4-byte signed integer for a variety of records.  This was changed
/// to 8-bytes after 3.0.  So, we need to do version-aware decoding.  Safely converts CdfInt4 to
/// CdfInt8 after decoding.
pub fn _decode_version3_int4_int8<R>(decoder: &mut Decoder<R>) -> Result<CdfInt8, DecodeError>
where
    R: io::Read + io::Seek,
{
    if decoder.version.major >= 3 {
        CdfInt8::decode_be(decoder)
    } else {
        let _s: i32 = CdfInt4::decode_be(decoder)?.into();
        Ok(CdfInt8::from(_s as i64))
    }
}
