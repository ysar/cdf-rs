#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::collection::RecordList,
    types::{CdfInt4, CdfInt8},
};

/// Possible child records of the Variable Index Record.
pub enum VariableIndexRecordChild {
    /// Contains a Variable Values record.
    VVR,
    /// Contains a Compressed Variable Values record.
    CVVR,
    /// Contains a lower-level Variable Index record.
    VXR,
}

/// Stores the contents of a Variable Index Record.
/// Variable Index Records are used in single-file CDFs to store the file offsets of any
/// lower level of VXRs, Variable Values Records, or Compressed Variable Value Records.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct VariableIndexRecord {
    /// Size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specification as an integer.
    pub record_type: CdfInt4,
    /// File offset pointing to the next VXR.
    pub vxr_next: Option<CdfInt8>,
    /// Number of entries in this VXR. Also the maximum number of VVR.
    pub num_entries: CdfInt4,
    /// The number of index entries actually used in this VXR.
    pub num_used_entries: CdfInt4,
    /// Record numbers of the first variable in VVRs or lower-level VXR.
    pub first: Vec<Option<CdfInt4>>,
    /// Record numbers of the last variable in VVRs or lower-level VXR.
    pub last: Vec<Option<CdfInt4>>,
    /// File offset of the VVR, CVVR or lower level VXR.
    pub offset: Vec<Option<CdfInt8>>,
}

impl Decodable for VariableIndexRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: std::io::Read + std::io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 6 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for VXR - expected 6, received {}",
                *record_type
            )));
        }
        let vxr_next = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let num_entries = CdfInt4::decode_be(decoder)?;
        let num_used_entries = CdfInt4::decode_be(decoder)?;

        let mut first: Vec<Option<CdfInt4>> = vec![None; usize::try_from(*num_entries)?];
        for val in first.iter_mut() {
            let x = CdfInt4::decode_be(decoder)?;
            if *x != -1 {
                // Actually checking for 0xFFFF_FFFF
                *val = Some(x);
            }
        }

        let mut last: Vec<Option<CdfInt4>> = vec![None; usize::try_from(*num_entries)?];
        for val in last.iter_mut() {
            let x = CdfInt4::decode_be(decoder)?;
            if *x != -1 {
                // Actually checking for 0xFFFF_FFFF
                *val = Some(x);
            }
        }

        let mut offset: Vec<Option<CdfInt8>> = vec![None; usize::try_from(*num_entries)?];
        for val in offset.iter_mut() {
            let x = decode_version3_int4_int8(decoder)?;
            if *x != -1 {
                // Actually checking for 0xFFFF_FFFF
                *val = Some(x);
            }
        }

        Ok(VariableIndexRecord {
            record_size,
            record_type,
            vxr_next,
            num_entries,
            num_used_entries,
            first,
            last,
            offset,
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, crate::error::CdfError>
    where
        R: std::io::Read + std::io::Seek,
    {
        unreachable!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}

impl RecordList for VariableIndexRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.vxr_next.clone()
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
    fn test_vxr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _vxr_example(file1)?;
        _vxr_example(file2)?;
        Ok(())
    }

    fn _vxr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        for vdr in cdf.cdr.gdr.rvdr_vec.iter() {
            assert_eq!(vdr.vxr_vec.len(), *cdf.cdr.gdr.num_rvars as usize);
        }
        for vdr in cdf.cdr.gdr.zvdr_vec.iter() {
            assert_eq!(vdr.vxr_vec.len(), *cdf.cdr.gdr.num_zvars as usize);
        }

        // if !cdf.rvxr_vec.is_empty() {
        //     dbg!(cdf.rvxr_vec);
        // }
        Ok(())
    }
}
