use semver::Version;

use crate::{
    decode::{Decodable, Decoder},
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
        let record_size = CdfInt8::decode(decoder)?;
        let record_type = CdfInt4::decode(decoder)?;
        let gdr_offset = CdfInt8::decode(decoder)?;
        let _version: i32 = CdfInt4::decode(decoder)?.into();
        let _release: i32 = CdfInt4::decode(decoder)?.into();
        let encoding: CdfEncoding = CdfInt4::decode(decoder)?.try_into()?;

        let _flags: i32 = CdfInt4::decode(decoder)?.into();
        let flags = CdrFlags {
            row_major: _flags & 0x00000001 == 1,
            single_file: _flags & 0x00000010 == 2,
            has_checksum: _flags & 0x00000100 == 4,
            md5_checksum: _flags & 0x00001000 == 8,
        };

        let rfu_a = CdfInt4::decode(decoder)?;
        let rfu_b = CdfInt4::decode(decoder)?;
        let _increment: i32 = CdfInt4::decode(decoder)?.into();

        let cdf_version = Version::new(_version as u64, _release as u64, _increment as u64);
        let identifier = CdfInt4::decode(decoder)?;
        let rfu_e = CdfInt4::decode(decoder)?;
        let mut copyright = if cdf_version < Version::new(2, 5, 0) {
            // read 1945 characters / bytes in ASCII
            vec![0u8; 1945]
        } else {
            vec![0u8; 256]
            // read 256 characters
        };
        _ = decoder.reader.read_exact(&mut copyright);
        let copyright: String = String::from_utf8(copyright)
            .map_err(|_| DecodeError::Other("Error decoding copyright notice.".to_string()))?;

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
#[derive(Debug)]
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
    use crate::repr::Endian;
    use std::fs::File;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn cdf_descriptor_record() -> Result<(), CdfError> {
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
        let cdf = cdf::Cdf::decode(&mut decoder)?;
        println!("{:?}", cdf);
        Ok(())
    }
}
