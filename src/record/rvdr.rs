#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::{
        collection::{get_record_vec, RecordList},
        vxr::VariableIndexRecord,
    },
    repr::Endian,
    types::{CdfInt4, CdfInt8, CdfString, CdfType},
};
use std::io;

/// Various options for rVariables.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct RVariableFlags {
    /// Whether this rVariable has variance.
    pub variance: bool,
    /// Whether this rVariables has padding.
    pub has_padding: bool,
    /// Whether this rVariable is compressed.
    pub is_compressed: bool,
}

/// Describes one rVariable stored in the CDF file.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct RVariableDescriptorRecord {
    /// Size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// File offset pointing to the next RVDR.
    pub rvdr_next: Option<CdfInt8>,
    /// Type of data stored in this rVariable.
    pub data_type: CdfInt4,
    /// Maximum record number stored in this rVariable.
    pub max_record: CdfInt4,
    /// File offset of the first Variable Index record.
    pub vxr_head: Option<CdfInt8>,
    /// File offset of the last Variable Index record.
    pub vxr_tail: Option<CdfInt8>,
    /// Boolean flags.
    pub flags: RVariableFlags,
    /// Type of sparse records.
    pub sparse_records: CdfInt4,
    /// This value is reserved for future use.
    pub rfu_b: CdfInt4,
    /// This value is reserved for future use.
    pub rfu_c: CdfInt4,
    /// This value is reserved for future use.
    pub rfu_f: CdfInt4,
    /// Number of elements of this data type in each variable value. Usually 1, for Chars this is
    /// the length of the string.
    pub num_elements: CdfInt4,
    /// Number (identifier) for this rVariable.
    pub num: CdfInt4,
    /// Offset for compression or sparse array.
    pub cpr_spr_offset: Option<CdfInt8>,
    /// Blocking factor (?)
    pub blocking_factor: CdfInt4,
    /// Name of this variable
    pub name: CdfString,
    /// Dimension variances for this variable.
    pub dim_variances: Vec<bool>,
    /// Pad value of this variable.
    pub pad_value: Vec<CdfType>,
    /// Vector of Variable Index Records.
    pub vxr_vec: Vec<VariableIndexRecord>,
}

impl Decodable for RVariableDescriptorRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 3 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for RVDR - expected 3, received {}",
                *record_type
            )));
        }

        let rvdr_next = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let data_type = CdfInt4::decode_be(decoder)?;
        let max_record = CdfInt4::decode_be(decoder)?;
        let vxr_head = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;
        let vxr_tail = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let flags = CdfInt4::decode_be(decoder)?;
        let flags = RVariableFlags {
            variance: *flags & 1i32 == 1,
            has_padding: *flags & 2i32 == 2,
            is_compressed: *flags & 4i32 == 4,
        };

        let sparse_records = CdfInt4::decode_be(decoder)?;

        let rfu_b = CdfInt4::decode_be(decoder)?;
        if *rfu_b != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_b read from file in RVDR - expected 0, received {}",
                *rfu_b
            )));
        }
        let rfu_c = CdfInt4::decode_be(decoder)?;
        if *rfu_c != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_c read from file in RVDR - expected -1, received {}",
                *rfu_c
            )));
        }
        let rfu_f = CdfInt4::decode_be(decoder)?;
        if *rfu_f != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_f read from file in RVDR - expected -1, received {}",
                *rfu_f
            )));
        }

        let num_elements = CdfInt4::decode_be(decoder)?;
        let num = CdfInt4::decode_be(decoder)?;

        // According to spec, this check should be with 0xFFFF_FFFF_FFFF_FFFF. But Rust
        // throws a compilation error because this does not fit inside a Int8. So we are
        // checking with -1 instead, which should lead to the same behavior.
        let cpr_spr_offset = decode_version3_int4_int8(decoder).map(|v| (*v != -1).then_some(v))?;

        let blocking_factor = CdfInt4::decode_be(decoder)?;

        let name = CdfString::decode_string_from_numbytes(decoder, 256)?;

        let num_r_dims = *decoder.context.get_num_dimension_rvariable()?;
        let mut dim_variances: Vec<bool> = vec![false; usize::try_from(num_r_dims)?];
        for d in dim_variances.iter_mut() {
            if *CdfInt4::decode_be(decoder)? == -1 {
                *d = true;
            }
        }

        let endianness = decoder.context.get_encoding()?.get_endian()?;
        let pad_value = match endianness {
            Endian::Big => CdfType::decode_vec_be(decoder, &data_type, &num_elements)?,
            Endian::Little => CdfType::decode_vec_le(decoder, &data_type, &num_elements)?,
        };

        let vxr_vec = if let Some(head) = &vxr_head {
            get_record_vec::<R, VariableIndexRecord>(decoder, head)?
        } else {
            vec![]
        };

        Ok(RVariableDescriptorRecord {
            record_size,
            record_type,
            rvdr_next,
            data_type,
            max_record,
            vxr_head,
            vxr_tail,
            flags,
            sparse_records,
            rfu_b,
            rfu_c,
            rfu_f,
            num_elements,
            num,
            cpr_spr_offset,
            blocking_factor,
            name,
            dim_variances,
            pad_value,
            vxr_vec,
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: std::io::Read + std::io::Seek,
    {
        unreachable!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}

impl RecordList for RVariableDescriptorRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.rvdr_next.clone()
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
    fn test_rvdr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _rvdr_example(file1, 0)?;
        _rvdr_example(file2, 15)?;
        Ok(())
    }

    fn _rvdr_example(filename: &str, rvdr_len: usize) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        assert_eq!(cdf.cdr.gdr.rvdr_vec.len(), rvdr_len);

        // if !cdf.rvdr_vec.is_empty() {
        //     dbg!(cdf.rvdr_vec);
        // }
        Ok(())
    }
}
