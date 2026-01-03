use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::collection::RecordList,
    types::{CdfInt4, CdfInt8, CdfString},
};
use std::io;

/// Various options for rVariables.
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
#[derive(Debug)]
pub struct RVariableDescriptorRecord {
    /// Size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// File offset pointing to the next RVDR.
    pub rvdr_next: Option<CdfInt8>,
    /// Type of data stored in this R Variable.
    pub data_type: CdfInt4,
    /// Maximum record number stored in this R variable.
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
    /// Number of elements of this data type.
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
    pub dim_variances: Vec<CdfInt4>,
    /// Pad value of this variable.
    pub pad_value: Vec<CdfInt4>,
}

impl Decodable for RVariableDescriptorRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
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

        let rvdr_next = {
            let v = decode_version3_int4_int8(decoder)?;
            if *v == 0 {
                None
            } else {
                Some(v)
            }
        };

        let data_type = CdfInt4::decode_be(decoder)?;
        let max_record = CdfInt4::decode_be(decoder)?;
        let vxr_head = {
            let v = decode_version3_int4_int8(decoder)?;
            if *v == 0 {
                None
            } else {
                Some(v)
            }
        };
        let vxr_tail = {
            let v = decode_version3_int4_int8(decoder)?;
            if *v == 0 {
                None
            } else {
                Some(v)
            }
        };

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

        let cpr_spr_offset = {
            let v = decode_version3_int4_int8(decoder)?;
            if *v == -1 {
                // According to spec, this check should be with 0xFFFF_FFFF_FFFF_FFFF. But Rust
                // throws a compilation error because this does not fit inside a Int8. So we are
                // checking with -1 instead, which should lead to the same behavior.
                None
            } else {
                Some(v)
            }
        };

        let blocking_factor = CdfInt4::decode_be(decoder)?;

        let name = CdfString::decode_string_from_numbytes(decoder, 256)?;

        let r_num_dims = *decoder.context.get_num_dimension_rvariable()?;
        let mut dim_variances = Vec::with_capacity(usize::try_from(r_num_dims)?);
        for _ in 0..r_num_dims {
            dim_variances.push(CdfInt4::decode_be(decoder)?)
        }

        let pad_value = vec![];
        if flags.has_padding {
            todo!();
        }

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
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self::Value, CdfError>
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

        _rvdr_example(file1)?;
        _rvdr_example(file2)?;
        Ok(())
    }

    fn _rvdr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        Ok(())
    }
}
