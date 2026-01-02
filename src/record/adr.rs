use crate::{
    decode::{Decodable, Decoder, _decode_version3_int4_int8},
    error::DecodeError,
    record::collection::RecordList,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// The Attribute Descriptor Record contains information on each attribute in the CDF.
#[derive(Debug)]
pub struct AttributeDescriptorRecord {
    pub record_size: CdfInt8,
    pub record_type: CdfInt4,
    pub adr_next: Option<CdfInt8>,
    pub agredr_head: Option<CdfInt8>,
    pub scope: CdfInt4,
    pub num: CdfInt4,
    pub num_gr_entries: CdfInt4,
    pub max_gr_entry: CdfInt4,
    pub rfu_a: CdfInt4,
    pub azedr_head: Option<CdfInt8>,
    pub num_z_entries: CdfInt4,
    pub max_z_entry: CdfInt4,
    pub rfu_e: CdfInt4,
    pub name: String,
}

impl Decodable for AttributeDescriptorRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = _decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 4 {
            return Err(DecodeError::Other(format!(
                "Invalid record_type for ADR - expected 4, received {}",
                *record_type
            )));
        };

        let adr_next = {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 {
                None
            } else {
                Some(_v)
            }
        };

        let agredr_head = {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 {
                None
            } else {
                Some(_v)
            }
        };

        let scope = CdfInt4::decode_be(decoder)?;
        let num = CdfInt4::decode_be(decoder)?;
        let num_gr_entries = CdfInt4::decode_be(decoder)?;
        let max_gr_entry = CdfInt4::decode_be(decoder)?;

        let rfu_a = CdfInt4::decode_be(decoder)?;
        if *rfu_a != 0 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_a read from file in ADR - expected 0, received {}",
                *rfu_a
            )));
        }

        let azedr_head = {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 {
                None
            } else {
                Some(_v)
            }
        };

        let num_z_entries = CdfInt4::decode_be(decoder)?;
        let max_z_entry = CdfInt4::decode_be(decoder)?;

        let rfu_e = CdfInt4::decode_be(decoder)?;
        if *rfu_e != -1 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_e read from file in ADR - expected -1, received {}",
                *rfu_e
            )));
        }

        let mut name = if decoder.version.major < 3 {
            vec![0u8; 64]
        } else {
            vec![0u8; 256]
        };
        _ = decoder.reader.read_exact(&mut name);
        let name: String = String::from_utf8(name.into_iter().take_while(|c| *c != 0).collect())
            .map_err(|e| DecodeError::Other(format!("Error decoding attribute name. - {e}")))?;

        Ok(AttributeDescriptorRecord {
            record_size,
            record_type,
            adr_next,
            agredr_head,
            scope,
            num,
            num_gr_entries,
            max_gr_entry,
            rfu_a,
            azedr_head,
            num_z_entries,
            max_z_entry,
            rfu_e,
            name,
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

impl RecordList for AttributeDescriptorRecord {
    fn next_record(&self) -> Option<CdfInt8> {
        self.adr_next.clone()
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
    fn test_adr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _ = _gdr_example(file1, 11)?;
        _ = _gdr_example(file2, 27)?;
        Ok(())
    }

    fn _gdr_example(filename: &str, adr_length: usize) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let adr = &cdf.adr_vec;
        // There are many ADR records in each file and I don't know how to validate each individually
        // so for now this hack of checking ADR lengths.
        assert_eq!(adr.len(), adr_length);
        Ok(())
    }
}
