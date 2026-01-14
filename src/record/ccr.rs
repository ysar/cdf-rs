#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// Stores compressed values in the case of full-file compression (as opposed to individual
/// variable data compression).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct CompressedCdfRecord {
    /// Size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// File offset of the compressed parameters record.
    pub cpr_offset: CdfInt8,
    /// Size of the CDF in its uncompressed form.
    pub uncompressed_size: CdfInt8,
    /// Reserved for future use.
    pub rfu_a: CdfInt4,
    /// Compressed CDF data as a vector of u8.
    pub data: Vec<u8>,
}

impl Decodable for CompressedCdfRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 10 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for CCR - expected 10, received {}",
                *record_type
            )));
        }
        let cpr_offset = decode_version3_int4_int8(decoder)?;
        let uncompressed_size = decode_version3_int4_int8(decoder)?;

        let rfu_a = CdfInt4::decode_be(decoder)?;
        if *rfu_a != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_a read from file in CCR - expected 0, received {}",
                *rfu_a
            )));
        }

        // Read the compressed data.
        // prior to v3.0 there were no 8-byte ints.
        let num_data = if decoder.context.get_version()?.major < 3 {
            usize::try_from(*record_size)? - 20
        } else {
            usize::try_from(*record_size)? - 32
        };
        let mut data = vec![0u8; num_data];
        decoder.reader.read_exact(&mut data)?;

        Ok(Self {
            record_size,
            record_type,
            cpr_offset,
            uncompressed_size,
            rfu_a,
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
    fn test_ccr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";

        _ccr_example(file1)?;
        Ok(())
    }

    fn _ccr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let _cdf = cdf::Cdf::decode_be(&mut decoder)?;
        // How to test the CCR?
        Ok(())
    }
}
