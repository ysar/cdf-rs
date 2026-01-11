use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::decode::{Decodable, Decoder};
use crate::error::CdfError;
use crate::record::cdr::CdfDescriptorRecord;
use crate::repr::CdfVersion;
use crate::types::CdfUint4;

/// General struct to hold the contents of the CDF file.
// #[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Cdf {
    /// Whether this CDF file is compressed.
    pub is_compressed: bool,
    /// Contents of the CDF Descriptor Record.
    pub cdr: CdfDescriptorRecord,
}

impl Cdf {
    /// Decode or deserialize a CDF file.
    pub fn read_cdf_file(file_path: PathBuf) -> Result<Self, CdfError> {
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        Cdf::decode_be(&mut decoder)
    }
}
impl Decodable for Cdf {
    /// Decode a value from the input that implements `io::Read`.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        // Decode the magic numbers.  The first number is not that important as it seems.
        let m1 = CdfUint4::decode_be(decoder)?;
        let m2 = CdfUint4::decode_be(decoder)?;

        // This is mostly a hack to get a hint of the CDF version. We read in the actual version
        // properly in the CDR. We need to know before reading the CDR if the CDF is >= v3.0 or
        // not.
        let version = match m1.into() {
            0xcdf30001 => CdfVersion::new(3, 0, 0),
            0xcdf26002 => CdfVersion::new(2, 6, 0),
            0x0000ffff => CdfVersion::new(2, 0, 0),
            v => return Err(CdfError::Decode(format!("Invalid magic number - {v}"))),
        };
        decoder.context.set_version(version);

        let is_compressed: bool = match m2.into() {
            0x0000ffffu32 => false,
            0xcccc0001u32 => true,
            v => return Err(CdfError::Decode(format!("Invalid magic number - {v}"))),
        };

        // Parse the CDF Descriptor Record that is present after the magic numbers.
        let cdr = CdfDescriptorRecord::decode_be(decoder)?;

        Ok(Cdf { is_compressed, cdr })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        unreachable!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::error::CdfError;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_read_cdf() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _cdf_example(file1)?;
        _cdf_example(file2)?;
        Ok(())
    }

    fn _cdf_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = Cdf::decode_be(&mut decoder)?;
        // dbg!(cdf);
        Ok(())
    }
}
