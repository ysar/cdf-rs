#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    repr::Endian,
    types::{CdfInt4, CdfInt8, CdfType},
};
use std::io;

/// A Variable Record contains an array of variables. Each variable may have multiple elements. For
/// example, a variable record may contain many strings. Each string is an element of the variable
/// array, and is a collection of CdfChars.
///
/// Information about each variable record is provided in the RVDR for rVariables and in the ZVDR
/// for zVariables. This information is copied over to the variable record associated with the
/// description.
///
/// Here is where we can get the necessary data.
///
/// For rVariables -
/// - `data_type` is stored in the rVDR as `data_type`
/// - `num_elements` is stored in the rVDR as `num_elements`
/// - `num_dims` is stored in the GDR as `num_r_dims` (shared for all rVariables)
/// - `size_dims` is stored in the GDR as `size_r_dims` (shared for all rVariables)
/// - `dim_variances` is stored in the rVDR as `dim_variances`
/// - `data` is stored in the VariableValuesRecord that we need to read in.
///
/// For zVariables -
/// - `data_type` is stored in the zVDR as `data_type`
/// - `num_elements` is stored in the zVDR as `num_elements`
/// - `num_dims` is stored in the zVDR as `num_z_dims`
/// - `size_dims` is stored in the zVDR as `size_z_dims`
/// - `dim_variances` is stored in the rVDR as `dim_variances`
/// - `data` is stored in the VariableValuesRecord that we need to read in.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct VariableRecord {
    /// Integer identifier for the data type stored in this variable record as per the spec.
    pub data_type: CdfInt4,
    /// Number of data of type `data_type` stored in this record.
    pub data_len: CdfInt4,
    /// The actual data stored in this variable record.
    pub data: Vec<CdfType>,
}

impl Decodable for VariableRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let data_type = decoder.context.get_var_data_type()?;
        let data_len = decoder.context.get_var_data_len()?;

        // Read in the values of this attribute based on the encoding specified in the CDR.
        let endianness = decoder.context.get_encoding()?.get_endian()?;
        let data = match endianness {
            Endian::Big => CdfType::decode_vec_be(decoder, &data_type, &data_len)?,
            Endian::Little => CdfType::decode_vec_le(decoder, &data_type, &data_len)?,
        };

        Ok(VariableRecord {
            data_type: data_type.clone(),
            data_len: data_len.clone(),
            data,
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

/// Stores the contents of a Variable Values Record.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct VariableValuesRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// Records (finally, the actual DATA that is stored in the CDF.). Each record contains an
    /// array of data. The number of such records, and the dimension of each array is stored either
    /// in the GDR or RVDR in the case of rVariables, or
    /// in the ZVDR in the case of zVariables.
    /// The attributes corresponding to these variables are stored in the AGREDR (for rVariables)
    /// and in the AZEDR (for zVariables).
    /// The number of records is the product of the number of elements in each record times the
    /// product of sizes of all actively stored dimensions.
    pub records: Vec<VariableRecord>,
}

impl Decodable for VariableValuesRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 7 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for VVR - expected 7, received {}",
                *record_type
            )));
        }

        let num_records = decoder.context.get_num_records()?;

        let mut records = Vec::with_capacity(num_records);
        for _ in 0..num_records {
            records.push(VariableRecord::decode_be(decoder)?);
        }

        Ok(VariableValuesRecord {
            record_size,
            record_type,
            records,
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
