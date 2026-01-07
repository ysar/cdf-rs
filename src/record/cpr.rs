#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    types::{CdfInt4, CdfInt8},
};
use std::{fmt, io};

/// Stores the different possible compressions that CDF files could make use of.
#[repr(i32)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum CdfCompressionKind {
    /// No compression
    None = 0,
    /// RLE compression
    Rle = 1,
    /// Huffman coding
    Huff = 2,
    /// Adaptive Huffman coding
    Ahuff = 3,
    /// Gzip compression
    Gzip = 5,
}

impl fmt::Display for CdfCompressionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, "CdfCompressionKind::None"),
            Self::Rle => write!(f, "CdfCompressionKind::Rle"),
            Self::Huff => write!(f, "CdfCompressionKind::Huff"),
            Self::Ahuff => write!(f, "CdfCompressionKind::Ahuff"),
            Self::Gzip => write!(f, "CdfCompressionKind::Gzip"),
        }
    }
}

impl TryFrom<i32> for CdfCompressionKind {
    type Error = CdfError;
    fn try_from(value: i32) -> Result<Self, CdfError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Rle),
            2 => Ok(Self::Huff),
            3 => Ok(Self::Ahuff),
            5 => Ok(Self::Gzip),
            e => Err(CdfError::Other(format!(
                "Invalid discriminant for CdfCompressionKin - {e}."
            ))),
        }
    }
}

/// Stores the contents of a Compressed Parameters Record. A CPR is pointed to by either the CCR
/// (in case of full compression of the CDF file) or the VDR (in case of compression on individual
/// variables).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct CompressedParametersRecord {
    /// Size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// The type of compression used.
    pub compression_type: CdfCompressionKind,
    /// Value reserved for future use.
    pub rfu_a: CdfInt4,
    /// Compression parameter count.
    pub compressed_parameter_count: CdfInt4,
    /// Compression level.
    pub compression_level: CdfInt4,
}

impl Decodable for CompressedParametersRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 11 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for CPR - expected 11, received {}",
                *record_type
            )));
        }

        let compression_type: i32 = CdfInt4::decode_be(decoder)?.into();
        let compression_type: CdfCompressionKind = compression_type.try_into()?;

        let compressed_parameter_count = CdfInt4::decode_be(decoder)?;

        let rfu_a = CdfInt4::decode_be(decoder)?;
        if *rfu_a != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_a read from file in CPR - expected 0, received {}",
                *rfu_a
            )));
        }

        let compression_level = CdfInt4::decode_be(decoder)?;

        match &compression_type {
            CdfCompressionKind::Gzip => {
                if *compression_level == 0 {
                    return Err(CdfError::Decode(
                        "Invalid compression level read for kind Gzip, expected range 1-9."
                            .to_string(),
                    ));
                }
            }
            k => {
                if *compression_level != 0 {
                    return Err(CdfError::Decode(format!(
                        "Invalid compression level read for kind {k}, expected 0."
                    )));
                }
            }
        }

        Ok(CompressedParametersRecord {
            record_size,
            record_type,
            compression_type,
            rfu_a,
            compressed_parameter_count,
            compression_level,
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek,
    {
        unimplemented!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::cdf;
    use crate::error::CdfError;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_cpr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";

        _cpr_example(file1)?;
        Ok(())
    }

    fn _cpr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        Ok(())
    }
}
