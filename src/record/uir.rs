#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::collection::RecordList,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// Stores the contents of an Unused Internal Record.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct UnusedInternalRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// Next UIR
    pub uir_next: Option<CdfInt8>,
    /// Preivous UIR
    pub uir_prev: Option<CdfInt8>,
    /// Remainder
    pub remainder: Vec<u8>,
}

impl Decodable for UnusedInternalRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for UIR - expected -1, received {}",
                *record_type
            )));
        }

        let uir_next = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;
        let uir_prev = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        // Read the remainder data.
        // prior to v3.0 there were no 8-byte ints.
        let num_data = if decoder.context.get_version()?.major < 3 {
            usize::try_from(*record_size)? - 16
        } else {
            usize::try_from(*record_size)? - 28
        };
        let mut remainder = vec![0u8; num_data];
        decoder.reader.read_exact(&mut remainder)?;

        Ok(UnusedInternalRecord {
            record_size,
            record_type,
            uir_next,
            uir_prev,
            remainder,
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

impl RecordList for UnusedInternalRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.uir_next.clone()
    }
}

/// Stores the contents of an Unsociable Unused Internal Record. (yes, that is the official name)
/// There are isolated unused records that are not stored on the unused linked-list.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct UnsociableUnusedInternalRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// Remainder
    pub remainder: Vec<u8>,
}

impl Decodable for UnsociableUnusedInternalRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for UUIR - expected -1, received {}",
                *record_type
            )));
        }

        // Read the remainder data.
        // prior to v3.0 there were no 8-byte ints.
        let num_data = if decoder.context.get_version()?.major < 3 {
            usize::try_from(*record_size)? - 8
        } else {
            usize::try_from(*record_size)? - 12
        };
        let mut remainder = vec![0u8; num_data];
        decoder.reader.read_exact(&mut remainder)?;

        Ok(UnsociableUnusedInternalRecord {
            record_size,
            record_type,
            remainder,
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
    fn test_uir_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";

        _uir_example(file1)?;
        Ok(())
    }

    fn _uir_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        assert_eq!(cdf.cdr.gdr.uir_vec.len(), 3);
        Ok(())
    }
}
