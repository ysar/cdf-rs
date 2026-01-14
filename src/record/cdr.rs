#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::gdr::GlobalDescriptorRecord,
    repr::{CdfEncoding, CdfVersion},
    types::{CdfInt4, CdfInt8, CdfString},
};
use std::io;

/// Flags pertaining to this CDF file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq)]
pub struct CdrFlags {
    /// Whether this is row_major (true) or column-major (false)
    pub row_major: bool,
    /// Whether this is a single file CDF, as opposed to multi-file CDFs.
    pub single_file: bool,
    /// Whether this CDF file has a checksum.
    pub has_checksum: bool,
    /// Whether the checksum is an MD5 checksum.
    pub md5_checksum: bool,
}

/// The CDF Descriptor Record is present in all CDF files at a file offset of 8-bytes and contains
/// general information about the CDF.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct CdfDescriptorRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// The file offset of the global descriptor record.
    pub gdr_offset: CdfInt8,
    /// The version of the CDF library used to create this file.
    pub cdf_version: CdfVersion,
    /// The encoding for data stored inside this CDF.
    pub encoding: CdfEncoding,
    /// Flags holds information on different options for this file.
    pub flags: CdrFlags,
    /// A value reserved for future use.
    pub rfu_a: CdfInt4,
    /// A value reserved for future use.
    pub rfu_b: CdfInt4,
    /// Identifier.
    pub identifier: CdfInt4,
    /// A value reserved for future use.
    pub rfu_e: CdfInt4,
    /// The copyright string.
    pub copyright: CdfString,
    /// Contents of the global descriptor record.
    pub gdr: GlobalDescriptorRecord,
}

impl Decodable for CdfDescriptorRecord {
    /// Decode the CDF Descriptor Record from the CDF file.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 1 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for CDR - expected 1, received {}",
                *record_type
            )));
        }

        let gdr_offset = decode_version3_int4_int8(decoder)?;
        let version: i32 = CdfInt4::decode_be(decoder)?.into();
        let release: i32 = CdfInt4::decode_be(decoder)?.into();
        let encoding: CdfEncoding = CdfInt4::decode_be(decoder)?.try_into()?;

        // Set the encoding of the decoder using the value read from the CDR.
        decoder.context.set_encoding(encoding.clone());
        decoder.context.set_endianness(encoding.get_endian()?);

        let flags = CdfInt4::decode_be(decoder)?;
        let flags = CdrFlags {
            row_major: *flags & 1i32 == 1,
            single_file: *flags & 2i32 == 2,
            has_checksum: *flags & 4i32 == 4,
            md5_checksum: *flags & 8i32 == 8,
        };

        decoder.context.set_row_majority(flags.row_major);

        let rfu_a = CdfInt4::decode_be(decoder)?;
        if *rfu_a != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_a read from file in CDR - expected 0, received {}",
                *rfu_a
            )));
        }
        let rfu_b = CdfInt4::decode_be(decoder)?;
        if *rfu_b != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_b read from file in CDR - expected 0, received {}",
                *rfu_b
            )));
        }

        let increment: i32 = CdfInt4::decode_be(decoder)?.into();

        let cdf_version = CdfVersion::new(
            u16::try_from(version)?,
            u16::try_from(release)?,
            u16::try_from(increment)?,
        );

        // Save the CDF version inside the decoder context for later use.
        decoder.context.set_version(cdf_version.clone());

        let identifier = CdfInt4::decode_be(decoder)?;
        let rfu_e = CdfInt4::decode_be(decoder)?;
        let copyright = if cdf_version < CdfVersion::new(2, 5, 0) {
            CdfString::decode_string_from_numbytes(decoder, 1945)?
        } else {
            CdfString::decode_string_from_numbytes(decoder, 256)?
        };

        let gdr = GlobalDescriptorRecord::decode_be(decoder)?;

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
            gdr,
        })
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

    use crate::cdf;
    use crate::error::CdfError;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_cdr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _cdf_descriptor_record_example(
            file1,
            312,
            320,
            CdfVersion::new(3, 8, 1),
            CdfEncoding::IbmPc,
            CdrFlags {
                row_major: true,
                single_file: true,
                has_checksum: true,
                md5_checksum: true,
            },
            143,
        )?;

        _cdf_descriptor_record_example(
            file2,
            304,
            312,
            CdfVersion::new(2, 5, 22),
            CdfEncoding::Network,
            CdrFlags {
                row_major: true,
                single_file: true,
                has_checksum: false,
                md5_checksum: false,
            },
            240,
        )?;
        Ok(())
    }

    fn _cdf_descriptor_record_example(
        filename: &str,
        record_size: i64,
        gdr_offset: i64,
        version: CdfVersion,
        encoding: CdfEncoding,
        flags: CdrFlags,
        len_copyright: usize,
    ) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let cdr = &cdf.cdr;
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
        assert_eq!(cdr.copyright.len(), len_copyright);
        Ok(())
    }
}
