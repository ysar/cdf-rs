use crate::decode::{decode_version3_int4_int8, Decodable, Decoder};
use crate::error::CdfError;
use crate::record::collection::RecordList;
use crate::repr::Endian;
use crate::types::{decode_cdf_type_be, decode_cdf_type_le, CdfInt4, CdfInt8, CdfType};
use std::io;

/// Struct to store contents of an Attribute Entry Descriptor Record that stores information on
/// zVariable attributes.
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
    /// The number of elements stored within this record.
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
    type Value = Self;

    /// Decode a value from the input that implements `io::Read`.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
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

        let azedr_next = {
            let v = decode_version3_int4_int8(decoder)?;
            if *v == 0 {
                None
            } else {
                Some(v)
            }
        };

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
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let adr_vec = &cdf.adr_vec;
        let azedr_vec_all = &cdf.azedr_vec;
        for (adr, azedr_vec) in adr_vec.iter().zip(azedr_vec_all) {
            assert_eq!(*adr.num_z_entries as usize, azedr_vec.len());
        }
        Ok(())
    }
}
