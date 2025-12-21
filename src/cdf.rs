use std::io;

use crate::decode::{Decodable, Decoder};
use crate::error::DecodeError;
use crate::record::{self, InternalRecord};
use crate::types::CdfUint4;

/// General struct to hold the contents of the CDF file.
#[derive(Debug)]
pub struct Cdf {
    pub is_compressed: bool,
    pub records: Vec<InternalRecord>,
}

impl Decodable for Cdf {
    type Value = Self;

    /// Decode a value from the input that implements `io::Read`.
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError> {
        // Decode the magic numbers.  The first number is not that important as it seems.
        let _ = CdfUint4::decode(decoder)?;
        let m2 = CdfUint4::decode(decoder)?;

        let is_compressed: bool = match m2.into() {
            0x0000ffffu32 => false,
            0xcccc0001u32 => true,
            v => return Err(DecodeError::InvalidMagicNumber(v)),
        };

        let mut cdf = Cdf {
            is_compressed,
            records: vec![],
        };

        // Parse the CDF Descriptor Record that is present after the magic numbers.
        let cdr = record::cdr::CdfDescriptorRecord::decode(decoder)?;

        cdf.records.push(InternalRecord::CDR(cdr));
        Ok(cdf)
    }
}

#[cfg(test)]
mod tests {

    use crate::error::CdfError;
    use crate::repr::Endian;
    use std::fs::File;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn read_magic_numbers() -> Result<(), CdfError> {
        let path_test_file: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            "data",
            "test_alltypes.cdf",
        ]
        .iter()
        .collect();

        let f = File::open(path_test_file)?;
        let mut decoder = Decoder::new(f, Endian::Big)?;
        let cdf = Cdf::decode(&mut decoder)?;
        assert_eq!(cdf.is_compressed, false);
        Ok(())
    }
}
