use std::io::{self, SeekFrom};

use crate::decode::{Decodable, Decoder};
use crate::error::CdfError;
use crate::record::adr::AttributeDescriptorRecord;
use crate::record::agredr::AttributeGREntryDescriptorRecord;
use crate::record::azedr::AttributeZEntryDescriptorRecord;
use crate::record::cdr::CdfDescriptorRecord;
use crate::record::collection::get_record_vec;
use crate::record::gdr::GlobalDescriptorRecord;
use crate::repr::CdfVersion;
use crate::types::CdfUint4;

/// General struct to hold the contents of the CDF file.
#[derive(Debug)]
pub struct Cdf {
    /// Whether this CDF file is compressed.
    pub is_compressed: bool,
    /// Contents of the CDF Descriptor Record.
    pub cdr: CdfDescriptorRecord,
    /// Contents of the Global Descriptor Record.
    pub gdr: GlobalDescriptorRecord,
    /// Vector storing all Attribute Descriptor Record.
    pub adr_vec: Vec<AttributeDescriptorRecord>,
    /// Vector of all Attribute Entry Descriptor Records of type G/R for each ADR.
    pub agredr_vec: Vec<Vec<AttributeGREntryDescriptorRecord>>,
    /// Vector of all Attribute Entry Descriptor Records of type Z for each ADR.
    pub azedr_vec: Vec<Vec<AttributeZEntryDescriptorRecord>>,
}

impl Decodable for Cdf {
    type Value = Self;

    /// Decode a value from the input that implements `io::Read`.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        // Decode the magic numbers.  The first number is not that important as it seems.
        let m1 = CdfUint4::decode_be(decoder)?;
        let m2 = CdfUint4::decode_be(decoder)?;

        // This is mostly a hack to get a hint of the CDF version. We read in the actual version
        // properly in the CDR.
        let version = match m1.into() {
            0xcdf30001 => CdfVersion::new(3, 0, 0),
            0xcdf26002 => CdfVersion::new(2, 6, 0),
            0x0000ffff => CdfVersion::new(2, 0, 0),
            v => return Err(CdfError::Decode(format!("Invalid magic number - {v}"))),
        };
        decoder.set_version(version);

        let is_compressed: bool = match m2.into() {
            0x0000ffffu32 => false,
            0xcccc0001u32 => true,
            v => return Err(CdfError::Decode(format!("Invalid magic number - {v}"))),
        };

        // Parse the CDF Descriptor Record that is present after the magic numbers.
        let cdr = CdfDescriptorRecord::decode_be(decoder)?;

        // Parse the Global Descriptor Record. The GDR can be present at any file offset, so we need
        // to `seek` to the `gdr_offset` value read in the CDR.
        _ = decoder
            .reader
            .seek(SeekFrom::Start(u64::try_from(*cdr.gdr_offset)?))?;

        let gdr = GlobalDescriptorRecord::decode_be(decoder)?;

        // There MAY be attribute descriptor records present. Collect these into a vec of ADRs.
        // They are stored in the CDF in a linked-list with each record pointing to the next.
        let adr_vec = if let Some(adr_head) = &gdr.adr_head {
            get_record_vec::<R, AttributeDescriptorRecord>(decoder, adr_head)?
        } else {
            vec![]
        };

        // There may be attribute entry descriptor records present in the form of a linked-list.
        // There are two types - the AGREDR and AZEDR.  Both types have separate linked-lists.
        // Each ADR may contain several linked-lists corresponding to each attribute, hence the
        // Vec<Vec<_>>.
        let mut agredr_vec = vec![];
        for adr in &adr_vec {
            let agredr_vec_this = if let Some(agredr_head) = &adr.agredr_head {
                get_record_vec::<R, AttributeGREntryDescriptorRecord>(decoder, agredr_head)?
            } else {
                vec![]
            };
            agredr_vec.push(agredr_vec_this);
        }

        let mut azedr_vec = vec![];
        for adr in &adr_vec {
            let azedr_vec_this = if let Some(azedr_head) = &adr.azedr_head {
                get_record_vec::<R, AttributeZEntryDescriptorRecord>(decoder, azedr_head)?
            } else {
                vec![]
            };
            azedr_vec.push(azedr_vec_this);
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
        assert!(!cdf.is_compressed);
        dbg!(cdf);
        Ok(())
    }
}
