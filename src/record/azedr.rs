#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::decode::{decode_version3_int4_int8, Decodable, Decoder};
use crate::error::CdfError;
use crate::record::collection::RecordList;
use crate::repr::Endian;
use crate::types::{CdfInt4, CdfInt8, CdfType};
use std::io;

/// Struct to store contents of an Attribute Entry Descriptor Record that stores information on
/// zVariable attributes.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct AttributeZEntryDescriptorRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// The file offset of the next AZEDR record.
    pub azedr_next: Option<CdfInt8>,
    /// The attribute number that this AZEDR correspond to.
    pub attr_num: CdfInt4,
    /// The type of data stored in this AZEDR stored as an integer identifier.
    pub data_type: CdfInt4,
    /// The numeric identifier for this AZEDR.
    pub num: CdfInt4,
    /// The number of elements stored within each value of this record. Usually 1, for Chars it is
    /// the length of the string.
    pub num_elements: CdfInt4,
    /// The number of strings stored within this record.
    pub num_strings: CdfInt4,
    /// A value reserved for future use.
    pub rfu_b: CdfInt4,
    /// A value reserved for future use.
    pub rfu_c: CdfInt4,
    /// A value reserved for future use.
    pub rfu_d: CdfInt4,
    /// A value reserved for future use.
    pub rfu_e: CdfInt4,
    /// The values stored inside this AZEDR.
    pub value: Vec<CdfType>,
}

impl Decodable for AttributeZEntryDescriptorRecord {
    /// Decode a value from the input that implements `io::Read`.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 9 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for AZEDR - expected 9, received {}",
                *record_type
            )));
        }

        let azedr_next = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let attr_num = CdfInt4::decode_be(decoder)?;
        let data_type = CdfInt4::decode_be(decoder)?;
        let num = CdfInt4::decode_be(decoder)?;
        let num_elements = CdfInt4::decode_be(decoder)?;
        let num_strings = CdfInt4::decode_be(decoder)?;

        let rfu_b = CdfInt4::decode_be(decoder)?;
        if *rfu_b != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_b read from file in AZEDR - expected 0, received {}",
                *rfu_b
            )));
        }
        let rfu_c = CdfInt4::decode_be(decoder)?;
        if *rfu_c != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_c read from file in AZEDR - expected 0, received {}",
                *rfu_c
            )));
        }
        let rfu_d = CdfInt4::decode_be(decoder)?;
        if *rfu_d != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_d read from file in AZEDR - expected -1, received {}",
                *rfu_d
            )));
        }
        let rfu_e = CdfInt4::decode_be(decoder)?;
        if *rfu_e != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_e read from file in AZEDR - expected -1, received {}",
                *rfu_e
            )));
        }

        // Read in the values of this attribute based on the encoding specified in the CDR.
        let endianness = decoder.context.get_encoding()?.get_endian()?;
        let value = match endianness {
            Endian::Big => CdfType::decode_vec_be(decoder, &data_type, &num_elements)?,
            Endian::Little => CdfType::decode_vec_le(decoder, &data_type, &num_elements)?,
        };

        Ok(AttributeZEntryDescriptorRecord {
            record_size,
            record_type,
            azedr_next,
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

    fn decode_le<R>(_: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        unreachable!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}

impl RecordList for AttributeZEntryDescriptorRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.azedr_next.clone()
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
    fn test_azedr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _azedr_example(file1)?;
        _azedr_example(file2)?;
        Ok(())
    }

    fn _azedr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let adr_vec = &cdf.cdr.gdr.adr_vec;
        for adr in adr_vec.iter() {
            assert_eq!(*adr.num_z_entries as usize, adr.azedr_vec.len());
        }
        Ok(())
    }
}
