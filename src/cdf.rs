use std::io::{self, SeekFrom};

use semver::Version;

use crate::decode::{Decodable, Decoder};
use crate::error::DecodeError;
use crate::record::adr::AttributeDescriptorRecord;
use crate::record::agredr::AttributeGREntryDescriptorRecord;
use crate::record::azedr::AttributeZEntryDescriptorRecord;
use crate::record::cdr::CdfDescriptorRecord;
use crate::record::gdr::GlobalDescriptorRecord;
use crate::types::CdfUint4;

/// General struct to hold the contents of the CDF file.
#[derive(Debug)]
pub struct Cdf {
    pub is_compressed: bool,
    pub cdr: CdfDescriptorRecord,
    pub gdr: GlobalDescriptorRecord,
    pub adr_vec: Vec<AttributeDescriptorRecord>,
    pub agredr_vec: Vec<AttributeGREntryDescriptorRecord>,
    pub azedr_vec: Vec<AttributeZEntryDescriptorRecord>,
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

        // This is mostly a hack to get a hint of the CDF version. We read in the actual version
        // properly in the CDR.
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

        // There MAY be attribute descriptor records present. Collect these into a vec of ADRs.
        // They are stored in the CDF in a linked-list with each record pointing to the next.
        let mut adr_vec = vec![];
        if let Some(adr_head) = &gdr.adr_head {
            let mut adr_next = adr_head.clone();
            loop {
                _ = decoder.reader.seek(SeekFrom::Start(*adr_next as u64))?;
                let adr = AttributeDescriptorRecord::decode_be(decoder)?;
                if let Some(_next) = adr.adr_next.clone() {
                    adr_vec.push(adr);
                    adr_next = _next;
                } else {
                    adr_vec.push(adr);
                    break;
                }
            }
        }

        // There may be attribute entry descriptor records present in the form of a linked-list.
        // There are two types - the AGREDR and AZEDR.  Both types have separate linked-lists.
        // Each ADR may contain several linked-lists corresponding to
        // attribute entries for that attribute.  Phew.  For now, let's flatten them all into one
        // Vec and later deal with which AEDR corresponds to which attribute (this info is stored
        // also in each AEDR)
        let mut agredr_vec = vec![];
        for adr in adr_vec.iter() {
            if let Some(agredr_head) = &adr.agredr_head {
                let mut agredr_next = agredr_head.clone();
                loop {
                    _ = decoder.reader.seek(SeekFrom::Start(*agredr_next as u64))?;
                    let agredr = AttributeGREntryDescriptorRecord::decode_be(decoder)?;
                    if let Some(_next) = agredr.agredr_next.clone() {
                        agredr_vec.push(agredr);
                        agredr_next = _next;
                    } else {
                        agredr_vec.push(agredr);
                        break;
                    }
                }
            }
        }

        // looks absolutely ugly but don't know what we could do.  Maybe move this to a separate function.
        let mut azedr_vec = vec![];
        for adr in adr_vec.iter() {
            if let Some(azedr_head) = &adr.azedr_head {
                let mut azedr_next = azedr_head.clone();
                loop {
                    _ = decoder.reader.seek(SeekFrom::Start(*azedr_next as u64))?;
                    let azedr = AttributeZEntryDescriptorRecord::decode_be(decoder)?;
                    if let Some(_next) = azedr.azedr_next.clone() {
                        azedr_vec.push(azedr);
                        azedr_next = _next;
                    } else {
                        azedr_vec.push(azedr);
                        break;
                    }
                }
            }
        }

        Ok(Cdf {
            is_compressed,
            cdr,
            gdr,
            adr_vec,
            agredr_vec,
            azedr_vec,
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, DecodeError>
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
