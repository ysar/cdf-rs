use semver::Version;

use crate::{
    decode::{Decodable, Decoder, _decode_version3_int4_int8},
    error::DecodeError,
    repr::CdfEncoding,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// The CDF Descriptor Record is present in all CDF files at a file offset of 8-bytes and contains
/// general information about the CDF.
#[derive(Debug)]
pub struct CdfDescriptorRecord {
    pub record_size: CdfInt8,
    pub record_type: CdfInt4,
    pub gdr_offset: CdfInt8,
    pub cdf_version: Version,
    pub encoding: CdfEncoding,
    pub flags: CdrFlags,
    pub rfu_a: CdfInt4,
    pub rfu_b: CdfInt4,
    pub identifier: CdfInt4,
    pub rfu_e: CdfInt4,
    pub copyright: String,
}

impl Decodable for CdfDescriptorRecord {
    type Value = Self;

    /// Decode the CDF Descriptor Record from the CDF file.
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError> {
        let record_size = _decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode(decoder)?;
        if *record_type != 1 {
            return Err(DecodeError::Other(format!(
                "Invalid record_type for CDR. Expected 1, Received {}",
                *record_type
            )));
        }

        let gdr_offset = _decode_version3_int4_int8(decoder)?;
        let _version: i32 = CdfInt4::decode(decoder)?.into();
        let _release: i32 = CdfInt4::decode(decoder)?.into();
        let encoding: CdfEncoding = CdfInt4::decode(decoder)?.try_into()?;

        let _flags: i32 = CdfInt4::decode(decoder)?.into();
        let flags = CdrFlags {
            row_major: _flags & 1i32 == 1,
            single_file: _flags & 2i32 == 2,
            has_checksum: _flags & 4i32 == 4,
            md5_checksum: _flags & 8i32 == 8,
        };

        let rfu_a = CdfInt4::decode(decoder)?;
        let rfu_b = CdfInt4::decode(decoder)?;
        let _increment: i32 = CdfInt4::decode(decoder)?.into();

        let cdf_version = Version::new(_version as u64, _release as u64, _increment as u64);
        if cdf_version != decoder.version {
            decoder.set_version(cdf_version.clone());
        }

        let identifier = CdfInt4::decode(decoder)?;
        let rfu_e = CdfInt4::decode(decoder)?;
        let mut copyright = if cdf_version < Version::new(2, 5, 0) {
            vec![0u8; 1945]
        } else {
            vec![0u8; 256]
        };
        _ = decoder.reader.read_exact(&mut copyright);
        let copyright: String = String::from_utf8(copyright)
            .map_err(|e| DecodeError::Other(format!("Error decoding copyright notice. - {e}")))?;

        Ok(CdfDescriptorRecord {
            record_size,
            record_type,
            gdr_offset,
            cdf_version,
            encoding,
            flags,
            rfu_a,
            rfu_b,
            identifier,
            rfu_e,
            copyright,
        })
    }
}

/// Flags pertaining to this CDF file.
#[derive(Debug, PartialEq)]
pub struct CdrFlags {
    pub row_major: bool,
    pub single_file: bool,
    pub has_checksum: bool,
    pub md5_checksum: bool,
}

#[cfg(test)]
mod tests {

    use crate::cdf;
    use crate::error::CdfError;
    use crate::record::InternalRecord;
    use crate::repr::Endian;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_cdr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _ = _cdf_descriptor_record_example(
            file1,
            312,
            320,
            Version::new(3, 8, 1),
            CdfEncoding::IbmPc,
            CdrFlags {
                row_major: true,
                single_file: true,
                has_checksum: true,
                md5_checksum: true,
            },
        )?;

        _ = _cdf_descriptor_record_example(
            file2,
            304,
            312,
            Version::new(2, 5, 22),
            CdfEncoding::Network,
            CdrFlags {
                row_major: true,
                single_file: true,
                has_checksum: false,
                md5_checksum: false,
            },
        )?;
        Ok(())
    }

    fn _cdf_descriptor_record_example(
        filename: &str,
        record_size: i64,
        gdr_offset: i64,
        version: Version,
        encoding: CdfEncoding,
        flags: CdrFlags,
    ) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader, Endian::Big, None)?;
        let cdf = cdf::Cdf::decode(&mut decoder)?;
        let record = &cdf.records[0];
        if let InternalRecord::CDR(cdr) = record {
            assert_eq!(*cdr.record_size, record_size);
            assert_eq!(*cdr.record_type, 1);
            assert_eq!(*cdr.gdr_offset, gdr_offset);
            assert_eq!(cdr.cdf_version, version);
            assert_eq!(cdr.encoding, encoding);
            assert_eq!(cdr.flags, flags,);
            assert_eq!(*cdr.rfu_a, 0);
            assert_eq!(*cdr.rfu_b, 0);
            assert_eq!(*cdr.identifier, -1);
            assert_eq!(*cdr.rfu_e, -1);
            assert!(cdr.copyright.len() == 256);
        } else {
            panic!("CDR not found.");
        }
        println!("{:?}", cdf);
        Ok(())
    }
}
