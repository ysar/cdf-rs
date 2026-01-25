#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::{
        agredr::AttributeGREntryDescriptorRecord,
        azedr::AttributeZEntryDescriptorRecord,
        collection::{get_record_vec, RecordList},
    },
    types::{CdfInt4, CdfInt8, CdfString},
};
use std::io;

/// The Attribute Descriptor Record contains information on each attribute in the CDF.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct AttributeDescriptorRecord {
    /// The size in bytes of this record.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// The file offset of the next ADR.
    pub adr_next: Option<CdfInt8>,
    /// The file offset of the first AGREDR corresponding to this ADR.
    pub agredr_head: Option<CdfInt8>,
    /// Scope.
    pub scope: CdfInt4,
    /// The numeric identifier for this attribute.
    pub num: CdfInt4,
    /// The number of GR attributes stored within this attribute.
    pub num_gr_entries: CdfInt4,
    /// The maximum GR entry.
    pub max_gr_entry: CdfInt4,
    /// A value reserved for future use.
    pub rfu_a: CdfInt4,
    /// The file offset of the first AZEDR corresponding to this ADR.
    pub azedr_head: Option<CdfInt8>,
    /// The number of Z attributes stored within this attribute.
    pub num_z_entries: CdfInt4,
    /// The maximum Z entry.
    pub max_z_entry: CdfInt4,
    /// A value reserved for future use.
    pub rfu_e: CdfInt4,
    /// Name of this attribute.
    pub name: CdfString,
    /// Store vec of AGREDRs associated with this attribute.
    pub agredr_vec: Vec<AttributeGREntryDescriptorRecord>,
    /// Store vec of AZEDRs associated with this attribute.
    pub azedr_vec: Vec<AttributeZEntryDescriptorRecord>,
}

impl Decodable for AttributeDescriptorRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let cdf_version = decoder.context.version()?;

        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 4 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for ADR - expected 4, received {}",
                *record_type
            )));
        }

        let adr_next = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;
        let agredr_head = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let scope = CdfInt4::decode_be(decoder)?;
        let num = CdfInt4::decode_be(decoder)?;
        let num_gr_entries = CdfInt4::decode_be(decoder)?;
        let max_gr_entry = CdfInt4::decode_be(decoder)?;

        let rfu_a = CdfInt4::decode_be(decoder)?;
        if *rfu_a != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_a read from file in ADR - expected 0, received {}",
                *rfu_a
            )));
        }

        let azedr_head = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let num_z_entries = CdfInt4::decode_be(decoder)?;
        let max_z_entry = CdfInt4::decode_be(decoder)?;

        let rfu_e = CdfInt4::decode_be(decoder)?;
        if *rfu_e != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_e read from file in ADR - expected -1, received {}",
                *rfu_e
            )));
        }

        let name = if cdf_version.major < 3 {
            CdfString::decode_string_from_numbytes(decoder, 64)?
        } else {
            CdfString::decode_string_from_numbytes(decoder, 256)?
        };

        let agredr_vec = match &agredr_head {
            Some(head) => get_record_vec::<R, AttributeGREntryDescriptorRecord>(decoder, head)?,
            None => vec![],
        };

        let azedr_vec = match &azedr_head {
            Some(head) => get_record_vec::<R, AttributeZEntryDescriptorRecord>(decoder, head)?,
            None => vec![],
        };

        Ok(AttributeDescriptorRecord {
            record_size,
            record_type,
            adr_next,
            agredr_head,
            scope,
            num,
            num_gr_entries,
            max_gr_entry,
            rfu_a,
            azedr_head,
            num_z_entries,
            max_z_entry,
            rfu_e,
            name,
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

impl RecordList for AttributeDescriptorRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.adr_next.clone()
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
    fn test_adr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _adr_example(file1, 11)?;
        _adr_example(file2, 27)?;
        Ok(())
    }

    fn _adr_example(filename: &str, adr_length: usize) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let adr_vec = &cdf.cdr.gdr.adr_vec;
        // There are many ADR records in each file and I don't know how to validate each individually
        // so for now this hack of checking the length of the ADR vector.
        assert_eq!(adr_vec.len(), adr_length);
        Ok(())
    }
}
