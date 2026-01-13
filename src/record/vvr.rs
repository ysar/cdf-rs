#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    repr::Endian,
    types::{
        decode_cdf_type_be, decode_cdf_type_le, CdfChar, CdfInt4, CdfInt8, CdfString, CdfType,
    },
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
    data_type: CdfInt4,
    num_elements: CdfInt4,
    num_dims: CdfInt4,
    size_dims: Vec<CdfInt4>,
    dim_variances: Vec<bool>,
    data: Vec<CdfType>,
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
    ///
    pub records: Vec<VariableRecord>,
}

impl Decodable for VariableValuesRecord {
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for UIR - expected -1, received {}",
                *record_type
            )));
        }

        // Read in the values of this attribute based on the encoding specified in the CDR.
        let endianness = decoder.context.get_encoding()?.get_endian()?;
        let records = match endianness {
            Endian::Big => CdfType::decode_vec_be(decoder, &data_type, &num_elements)?,
            Endian::Little => CdfType::decode_vec_le(decoder, &data_type, &num_elements)?,
        };

        Ok(VariableValuesRecord {
            record_size,
            record_type,
            records,
        })
    }

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        unimplemented!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}
