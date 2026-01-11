#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// Stores the contents of a Compressed Variable Values record, which stores one section of
/// compressed variable value records (VVR).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct CompressedVariableValuesRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// Value reserved for future use.
    pub rfu_a: CdfInt4,
    /// Size in bytes of the post-compressed data.
    pub compressed_size: CdfInt8,
    /// Compressed data
    pub data: Vec<u8>,
}

impl Decodable for CompressedVariableValuesRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 13 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for CVVR - expected 13, received {}",
                *record_type
            )));
        }

        let rfu_a = CdfInt4::decode_be(decoder)?;
        if *rfu_a != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_a read from file in CVVR - expected 0, received {}",
                *rfu_a
            )));
        }

        let compressed_size = decode_version3_int4_int8(decoder)?;

        // Read the compressed data.
        // prior to v3.0 there were no 8-byte ints.
        let mut data = vec![0u8; usize::try_from(*compressed_size)?];
        decoder.reader.read_exact(&mut data)?;

        Ok(Self {
            record_size,
            record_type,
            rfu_a,
            compressed_size,
            data,
        })
    }
    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, CdfError>
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
    fn test_cvvr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";

        _cvvr_example(file1)?;
        Ok(())
    }

    fn _cvvr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        // How to test the CVVR?
        Ok(())
    }
}
