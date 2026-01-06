use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    record::collection::RecordList,
    repr::Endian,
    types::{decode_cdf_type_be, decode_cdf_type_le, CdfInt4, CdfInt8, CdfString, CdfType},
};
use std::io;

/// Various options for rVariables.
#[derive(Debug)]
pub struct ZVariableFlags {
    /// Whether this rVariable has variance.
    pub variance: bool,
    /// Whether this rVariables has padding.
    pub has_padding: bool,
    /// Whether this rVariable is compressed.
    pub is_compressed: bool,
}

/// Describes one zVariable stored in the CDF file.
#[derive(Debug)]
pub struct ZVariableDescriptorRecord {
    /// Size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// File offset pointing to the next RVDR.
    pub zvdr_next: Option<CdfInt8>,
    /// Type of data stored in this zVariable.
    pub data_type: CdfInt4,
    /// Maximum record number stored in this zvariable.
    pub max_record: CdfInt4,
    /// File offset of the first Variable Index record.
    pub vxr_head: Option<CdfInt8>,
    /// File offset of the last Variable Index record.
    pub vxr_tail: Option<CdfInt8>,
    /// Boolean flags.
    pub flags: ZVariableFlags,
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
    /// Number of dimensions for this zVariable.
    pub num_z_dims: CdfInt4,
    /// Vec of sizes for this zVariable.
    pub size_z_dims: Vec<CdfInt4>,
    /// Dimension variances for this variable.
    pub dim_variances: Vec<bool>,
    /// Pad value of this variable.
    pub pad_value: Vec<CdfType>,
}

impl Decodable for ZVariableDescriptorRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 8 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for ZVDR - expected 8, received {}",
                *record_type
            )));
        }

        let zvdr_next = {
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
        let flags = ZVariableFlags {
            variance: *flags & 1i32 == 1,
            has_padding: *flags & 2i32 == 2,
            is_compressed: *flags & 4i32 == 4,
        };

        let sparse_records = CdfInt4::decode_be(decoder)?;

        let rfu_b = CdfInt4::decode_be(decoder)?;
        if *rfu_b != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_b read from file in ZVDR - expected 0, received {}",
                *rfu_b
            )));
        }
        let rfu_c = CdfInt4::decode_be(decoder)?;
        if *rfu_c != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_c read from file in ZVDR - expected -1, received {}",
                *rfu_c
            )));
        }
        let rfu_f = CdfInt4::decode_be(decoder)?;
        if *rfu_f != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_f read from file in ZVDR - expected -1, received {}",
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

        let num_z_dims = CdfInt4::decode_be(decoder)?;
        let mut size_z_dims = Vec::with_capacity(usize::try_from(*num_z_dims)?);
        for _ in 0..*num_z_dims {
            size_z_dims.push(CdfInt4::decode_be(decoder)?)
        }

        let mut dim_variances = Vec::with_capacity(usize::try_from(*num_z_dims)?);
        for _ in 0..*num_z_dims {
            dim_variances.push(CdfInt4::decode_be(decoder).map(|x| *x == -1)?)
        }

        let pad_value = if flags.has_padding {
            // Read in the values of this attribute based on the encoding specified in the CDR.
            let mut value: Vec<CdfType> = Vec::with_capacity(usize::try_from(*num_elements)?);
            let endianness = decoder.context.get_encoding()?.get_endian()?;

            match endianness {
                Endian::Big => {
                    for _ in 0..*num_elements {
                        value.push(decode_cdf_type_be(decoder, *data_type)?);
                    }
                }
                Endian::Little => {
                    for _ in 0..*num_elements {
                        value.push(decode_cdf_type_le(decoder, *data_type)?);
                    }
                }
            }
            value
        } else {
            vec![]
        };

        Ok(ZVariableDescriptorRecord {
            record_size,
            record_type,
            zvdr_next,
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
            num_z_dims,
            size_z_dims,
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

impl RecordList for ZVariableDescriptorRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.zvdr_next.clone()
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
    fn test_zvdr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _zvdr_example(file1, 21)?;
        _zvdr_example(file2, 0)?;
        Ok(())
    }

    fn _zvdr_example(filename: &str, zvdr_len: usize) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        assert_eq!(cdf.zvdr_vec.len(), zvdr_len);
        // dbg!(cdf.zvdr_vec.len());
        Ok(())
    }
}
