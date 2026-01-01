use std::io::{self, SeekFrom};

use semver::Version;

use crate::decode::{Decodable, Decoder};
use crate::error::DecodeError;
use crate::record::adr::AttributeDescriptorRecord;
use crate::record::cdr::CdfDescriptorRecord;
use crate::record::gdr::GlobalDescriptorRecord;
use crate::types::CdfUint4;

/// General struct to hold the contents of the CDF file.
#[derive(Debug)]
pub struct Cdf {
    pub is_compressed: bool,
    pub cdr: CdfDescriptorRecord,
    pub gdr: GlobalDescriptorRecord,
    pub adr: Vec<AttributeDescriptorRecord>,
}

impl Decodable for Cdf {
    type Value = Self;

    /// Decode a value from the input that implements `io::Read`.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError>
    where
        R: io::Read + io::Seek,
    {
        // Decode the magic numbers.  The first number is not that important as it seems.
        let m1 = CdfUint4::decode_be(decoder)?;
        let m2 = CdfUint4::decode_be(decoder)?;

        let version = match m1.into() {
            0xcdf30001 => Version::new(3, 0, 0),
            0xcdf26002 => Version::new(2, 6, 0),
            0x0000ffff => Version::new(2, 0, 0),
            v => return Err(DecodeError::InvalidMagicNumber(v)),
        };
        decoder.set_version(version);

        let is_compressed: bool = match m2.into() {
            0x0000ffffu32 => false,
            0xcccc0001u32 => true,
            v => return Err(DecodeError::InvalidMagicNumber(v)),
        };

        // Parse the CDF Descriptor Record that is present after the magic numbers.
        let cdr = CdfDescriptorRecord::decode_be(decoder)?;

        // Parse the Global Descriptor Record. The GDR can be present at any file offset, so we need
        // to `seek` to the `gdr_offset` value read in the CDR.
        _ = decoder
            .reader
            .seek(SeekFrom::Start(*cdr.gdr_offset as u64))?;

        let gdr = GlobalDescriptorRecord::decode_be(decoder)?;

        // There MAY be an attribute descriptor record present. Collect these into a vec of ADRs.
        // They are stored in the CDF in a linked-list with each record pointing to the next.
        let mut adr = vec![];
        if let Some(adr_head) = &gdr.adr_head {
            let mut adr_next = adr_head.clone();
            loop {
                _ = decoder.reader.seek(SeekFrom::Start(*adr_next as u64))?;
                let _adr = AttributeDescriptorRecord::decode_be(decoder)?;
                if let Some(_a) = _adr.adr_next.clone() {
                    adr.push(_adr);
                    adr_next = _a;
                } else {
                    adr.push(_adr);
                    break;
                }
            }
        }

        Ok(Cdf {
            is_compressed,
            cdr,
            gdr,
            adr,
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, DecodeError>
    where
        R: io::Read + io::Seek,
    {
        Err(DecodeError::Other(
            "Little-endian decoding is not supported for records, only for values within records."
                .to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {

    use crate::error::CdfError;
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
        let mut decoder = Decoder::new(f)?;
        let cdf = Cdf::decode_be(&mut decoder)?;
        assert_eq!(cdf.is_compressed, false);
        dbg!(cdf);
        Ok(())
    }
}
