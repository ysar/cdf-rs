#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
    decode::{decode_version3_int4_int8, Decodable, Decoder},
    error::CdfError,
    repr::CdfVersion,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// The Global Descriptor Record is present in all uncompressed CDF files after the CDF Descriptor
/// Record, at the file offset noted in the CDR `gdr_offset` attribute.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct GlobalDescriptorRecord {
    /// The size of this record in bytes.
    pub record_size: CdfInt8,
    /// The type of record as defined in the CDF specfication as an integer.
    pub record_type: CdfInt4,
    /// The file-offset of the first R Variable Descriptor Record.
    pub rvdr_head: Option<CdfInt8>,
    /// The file-offset of the first Z Variable Descriptor Record.
    pub zvdr_head: Option<CdfInt8>,
    /// The file-offset of the first Attribute Descriptor Record.
    pub adr_head: Option<CdfInt8>,
    /// The file-offset representing the end-of-file.
    pub eof: Option<CdfInt8>,
    /// Number of R variables.
    pub num_rvars: CdfInt4,
    /// Number of attributes.
    pub num_attributes: CdfInt4,
    /// Maximum R variable.
    pub max_rvar: CdfInt4,
    /// Number of dimensions for R variables (Note: all R variables have the same dimension.)
    pub num_r_dims: CdfInt4,
    /// Number of Z variables.
    pub num_zvars: CdfInt4,
    /// The file offset for the Unused Internal Record.
    pub uir_head: Option<CdfInt8>,
    /// A value reserved for future use.
    pub rfu_c: CdfInt4,
    /// Date of last leapsecond update.
    pub date_last_leapsecond_update: CdfInt4,
    /// A value reserved for future use.
    pub rfu_e: CdfInt4,
    /// Sizes for R variables.
    pub size_r_dims: Vec<CdfInt4>,
}

impl Decodable for GlobalDescriptorRecord {
    type Value = Self;

    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let cdf_version = decoder.context.get_version()?;

        let record_size = decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode_be(decoder)?;
        if *record_type != 2 {
            return Err(CdfError::Decode(format!(
                "Invalid record_type for GDR - expected 2, received {}",
                *record_type
            )));
        }

        let rvdr_head = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;
        let zvdr_head = decode_version3_int4_int8(decoder)
            .map(|v| (*v != 0 && cdf_version >= CdfVersion::new(2, 2, 0)).then_some(v))?;

        let adr_head = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        // eof is undefined for CDF < v2.1
        let eof = decode_version3_int4_int8(decoder)
            .map(|eof| (cdf_version >= CdfVersion::new(2, 1, 0)).then_some(eof))?;

        let num_rvars = CdfInt4::decode_be(decoder)?;
        let num_attributes = CdfInt4::decode_be(decoder)?;
        let max_rvar = CdfInt4::decode_be(decoder)?;

        let num_r_dims = CdfInt4::decode_be(decoder)?;
        decoder
            .context
            .set_num_dimension_rvariable(num_r_dims.clone());

        let num_zvars = CdfInt4::decode_be(decoder)?;
        let uir_head = decode_version3_int4_int8(decoder).map(|v| (*v != 0).then_some(v))?;

        let rfu_c = CdfInt4::decode_be(decoder)?;
        if *rfu_c != 0 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_c read from file - expected 0, received {}",
                *rfu_c
            )));
        }

        let date_last_leapsecond_update = CdfInt4::decode_be(decoder)?;

        let rfu_e = CdfInt4::decode_be(decoder)?;
        if *rfu_e != -1 {
            return Err(CdfError::Decode(format!(
                "Invalid rfu_e read from file - expected -1, received {}",
                *rfu_e
            )));
        }

        let mut sizes_rvar = vec![CdfInt4::from(0); usize::try_from(*num_r_dims)?];
        for s in sizes_rvar.iter_mut() {
            // If there are rVariables present, read in their dimensions.
            *s = CdfInt4::decode_be(decoder)?;
        }

        Ok(Self {
            record_size,
            record_type,
            rvdr_head,
            zvdr_head,
            adr_head,
            eof,
            num_rvars,
            num_attributes,
            max_rvar,
            num_r_dims,
            num_zvars,
            uir_head,
            rfu_c,
            date_last_leapsecond_update,
            rfu_e,
            size_r_dims: sizes_rvar,
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

#[cfg(test)]
mod tests {

    use crate::cdf;
    use crate::error::CdfError;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_gdr_examples() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        let expected1 = GlobalDescriptorRecord {
            record_size: CdfInt8::from(84),
            record_type: CdfInt4::from(2),
            rvdr_head: None,
            zvdr_head: Some(CdfInt8::from(3968)),
            adr_head: Some(CdfInt8::from(404)),
            eof: Some(CdfInt8::from(117_050)),
            num_rvars: CdfInt4::from(0),
            num_attributes: CdfInt4::from(11),
            max_rvar: CdfInt4::from(-1),
            num_r_dims: CdfInt4::from(0),
            num_zvars: CdfInt4::from(21),
            uir_head: Some(CdfInt8::from(10964)),
            rfu_c: CdfInt4::from(0),
            date_last_leapsecond_update: CdfInt4::from(20_170_101),
            rfu_e: CdfInt4::from(-1),
            size_r_dims: vec![],
        };
        let expected2 = GlobalDescriptorRecord {
            record_size: CdfInt8::from(64),
            record_type: CdfInt4::from(2),
            rvdr_head: Some(CdfInt8::from(4405)),
            zvdr_head: None,
            adr_head: Some(CdfInt8::from(376)),
            eof: Some(CdfInt8::from(8_420_394)),
            num_rvars: CdfInt4::from(15),
            num_attributes: CdfInt4::from(27),
            max_rvar: CdfInt4::from(134_639),
            num_r_dims: CdfInt4::from(1),
            num_zvars: CdfInt4::from(0),
            uir_head: None,
            rfu_c: CdfInt4::from(0),
            date_last_leapsecond_update: CdfInt4::from(-1),
            rfu_e: CdfInt4::from(-1),
            size_r_dims: vec![CdfInt4::from(3)],
        };
        _gdr_example(file1, expected1)?;
        _gdr_example(file2, expected2)?;
        Ok(())
    }

    fn _gdr_example(filename: &str, exp: GlobalDescriptorRecord) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;
        let cdf = cdf::Cdf::decode_be(&mut decoder)?;
        let gdr = &cdf.gdr;
        assert_eq!(gdr.record_size, exp.record_size);
        assert_eq!(gdr.record_size, exp.record_size);
        assert_eq!(gdr.record_type, exp.record_type);
        assert_eq!(gdr.rvdr_head, exp.rvdr_head);
        assert_eq!(gdr.zvdr_head, exp.zvdr_head);
        assert_eq!(gdr.adr_head, exp.adr_head);
        assert_eq!(gdr.eof, exp.eof);
        assert_eq!(gdr.num_rvars, exp.num_rvars);
        assert_eq!(gdr.num_attributes, exp.num_attributes);
        assert_eq!(gdr.max_rvar, exp.max_rvar);
        assert_eq!(gdr.num_r_dims, exp.num_r_dims);
        assert_eq!(gdr.num_zvars, exp.num_zvars);
        assert_eq!(gdr.uir_head, exp.uir_head);
        assert_eq!(gdr.rfu_c, exp.rfu_c);
        assert_eq!(
            gdr.date_last_leapsecond_update,
            exp.date_last_leapsecond_update
        );
        assert_eq!(gdr.rfu_e, exp.rfu_e);
        assert_eq!(gdr.size_r_dims.len(), exp.size_r_dims.len());
        for i in 0..gdr.size_r_dims.len() {
            assert_eq!(gdr.size_r_dims[i], exp.size_r_dims[i]);
        }
        Ok(())
    }
}
