use crate::decode::{Decodable, Decoder, _decode_version3_int4_int8};
use crate::error::DecodeError;
use crate::record::collection::RecordList;
use crate::repr::Endian;
use crate::types::{decode_cdf_type_be, decode_cdf_type_le, CdfInt4, CdfInt8, CdfType};
use std::io;

#[derive(Debug)]
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
                "Invalid rfu_b read from file in AGREDR - expected 0, received {}",
                *rfu_b
            )));
        }
        let rfu_c = CdfInt4::decode_be(decoder)?;
        if *rfu_c != 0 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_c read from file in AGREDR - expected 0, received {}",
                *rfu_c
            )));
        }
        let rfu_d = CdfInt4::decode_be(decoder)?;
        if *rfu_d != -1 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_d read from file in AGREDR - expected -1, received {}",
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

        // Read in the values of this attribute based on the encoding specified in the CDR.
        let mut value: Vec<CdfType> = Vec::with_capacity(*num_elements as usize);
        let endianness = decoder.encoding.get_endian()?;

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
        unreachable!(
            "Little-endian decoding is not supported for records, only for values within records."
        )
    }
}

impl RecordList for AttributeGREntryDescriptorRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.agredr_next.clone()
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
    fn test_agredr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _ = _agredr_example(file1)?;
        _ = _agredr_example(file2)?;
        Ok(())
    }

    fn _agredr_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let adr_vec = &cdf.adr_vec;
        let agredr_vec_all = &cdf.agredr_vec;
        for (adr, agredr_vec) in adr_vec.iter().zip(agredr_vec_all) {
            assert_eq!(*adr.num_gr_entries as usize, agredr_vec.len());
        }
        Ok(())
    }
}
