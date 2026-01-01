use crate::decode::{Decodable, Decoder, _decode_version3_int4_int8};
use crate::error::DecodeError;
use crate::types::{CdfInt4, CdfInt8, CdfType};
use std::io;

pub struct AttributeGREntryDescriptorRecord {
    pub record_size: CdfInt8,
    pub record_type: CdfInt4,
    pub agredr_next: Option<CdfInt8>,
    pub attr_num: CdfInt4,
    pub data_type: CdfInt4,
    pub num: CdfInt4,
    pub num_elements: CdfInt4,
    pub num_strings: CdfInt4,
    pub rfu_b: CdfInt4,
    pub rfu_c: CdfInt4,
    pub rfu_d: CdfInt4,
    pub rfu_e: CdfInt4,
    pub value: Vec<CdfType>,
}

impl Decodable for AttributeGREntryDescriptorRecord {
    type Value = Self;

    /// Decode a value from the input that implements `io::Read`.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = _decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 5 {
            return Err(DecodeError::Other(format!(
                "Invalid record_type for AGREDR - expected 5, received {}",
                *record_type
            )));
        };

        let agredr_next = {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 {
                None
            } else {
                Some(_v)
            }
        };

        let attr_num = CdfInt4::decode_be(decoder)?;
        let data_type = CdfInt4::decode_be(decoder)?;
        let num = CdfInt4::decode_be(decoder)?;
        let num_elements = CdfInt4::decode_be(decoder)?;
        let num_strings = CdfInt4::decode_be(decoder)?;

        let rfu_b = CdfInt4::decode_be(decoder)?;
        if *rfu_b != 0 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_e read from file in AGREDR - expected 0, received {}",
                *rfu_b
            )));
        }
        let rfu_c = CdfInt4::decode_be(decoder)?;
        if *rfu_c != 0 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_e read from file in AGREDR - expected 0, received {}",
                *rfu_c
            )));
        }
        let rfu_d = CdfInt4::decode_be(decoder)?;
        if *rfu_d != -1 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_e read from file in AGREDR - expected -1, received {}",
                *rfu_d
            )));
        }
        let rfu_e = CdfInt4::decode_be(decoder)?;
        if *rfu_e != -1 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_e read from file in AGREDR - expected -1, received {}",
                *rfu_e
            )));
        }

        let value: Vec<CdfType> = Vec::with_capacity(*num_elements as usize);

        // The `values` may have a different encoding (endianness) from the CDF file, and this is
        // determined by the encoding of the CDF that the reader is aware of.

        Ok(AttributeGREntryDescriptorRecord {
            record_size,
            record_type,
            agredr_next,
            attr_num,
            data_type,
            num,
            num_elements,
            num_strings,
            rfu_b,
            rfu_c,
            rfu_d,
            rfu_e,
            value,
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
