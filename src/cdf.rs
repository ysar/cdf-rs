use semver::Version;
use std::io;

use crate::error::DecodeError;
use crate::record::{self, InternalRecord};
use crate::traits::{Decode, Encode};

/// General struct to hold the contents of the CDF file.
pub struct Cdf {
    pub version: Version,
    pub is_compressed: bool,
    pub records: Vec<InternalRecord>,
}

impl<R: io::Read> Decode<R> for Cdf {
    type Output = Cdf;

    fn decode(mut reader: R) -> Result<Self::Output, DecodeError> {
        // Decode the magic numbers.
        let m1 = u32::decode(&mut reader)?;
        let m2 = u32::decode(&mut reader)?;

        let version: Version = match m1 {
            0x0000ffffu32 => {
                Version::parse("2.5.0").map_err(|err| DecodeError::Other(format!("{err}")))?
            }
            0xcdf30001u32 => {
                Version::parse("2.6.0").map_err(|err| DecodeError::Other(format!("{err}")))?
            }
            _ => return Err(DecodeError::InvalidMagicNumber(m1)),
        };

        let is_compressed: bool = match m2 {
            0x0000ffffu32 => false,
            0xcccc0001u32 => true,
            _ => return Err(DecodeError::InvalidMagicNumber(m2)),
        };

        //let cdr = Cdf::decode_cdr(reader);
        //let gdr = Cdf::decode_gdr(reader);
        //let records = Cdf::decode_internal_records(reader);

        let cdf = Cdf {
            version,
            is_compressed,
            records: vec![],
        };
        Ok(cdf)
    }
}

//impl Cdf {
//    /// Read the magic numbers from the CDF file. They are two 32-bit unsigned integers at the
//    /// beginning of the file record (i.e. at file offset 0 and 4 bytes).
//    pub fn read_magic_numbers(mut reader: impl io::Read) -> Result<(u32, u32), io::Error> {
//        let mut buf = [0u8; 4];
//
//        reader.read_exact(&mut buf[..])?;
//        let magic_num1 = u32::from_be_bytes(buf);
//
//        reader.read_exact(&mut buf[..])?;
//        let magic_num2 = u32::from_be_bytes(buf);
//
//        Ok((magic_num1, magic_num2))
//    }
//
//    // Decode the CDF Descriptor Record
//    fn decode_cdr(mut reader: impl io::Read) -> Result<InternalRecord, DecodeError> {
//        record::InternalRecord::CDR::decode(reader)
//    }
//}

//#[cfg(test)]
//mod tests {
//
//    use std::fs::File;
//    use std::path::PathBuf;
//
//    use super::*;
//
//    #[test]
//    fn read_magic_numbers() {
//        let path_test_file: PathBuf = [
//            env!("CARGO_MANIFEST_DIR"),
//            "tests",
//            "data",
//            "test_alltypes.cdf",
//        ]
//        .iter()
//        .collect();
//
//        let f = File::open(path_test_file).unwrap();
//        let cdf = Cdf::decode(f).unwrap();
//        assert_eq!(cdf.is_compressed, false);
//        assert_eq!(cdf.version, Version::parse("2.6").unwrap());
//    }
//}
