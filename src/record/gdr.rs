use semver::Version;

use crate::{
    decode::{Decodable, Decoder, _decode_version3_int4_int8},
    error::DecodeError,
    types::{CdfInt4, CdfInt8},
};
use std::io;

/// The Global Descriptor Record is present in all uncompressed CDF files after the CDF Descriptor
/// Record, at the file offset noted in the CDR `gdr_offset` attribute.
#[derive(Debug)]
pub struct GlobalDescriptorRecord {
    pub record_size: CdfInt8,
    pub record_type: CdfInt4,
    pub rvdr_head: Option<CdfInt8>,
    pub zvdr_head: Option<CdfInt8>,
    pub adr_head: Option<CdfInt8>,
    pub eof: Option<CdfInt8>,
    pub num_rvars: CdfInt4,
    pub num_attributes: CdfInt4,
    pub max_rvar: CdfInt4,
    pub dim_rvar: CdfInt4,
    pub num_zvars: CdfInt4,
    pub uir_head: CdfInt8,
    pub rfu_c: CdfInt4,
    pub date_last_leapsecond_update: CdfInt4,
    pub rfu_e: CdfInt4,
    pub sizes_rvar: Box<[CdfInt4]>,
}

impl Decodable for GlobalDescriptorRecord {
    type Value = Self;

    fn decode<R>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>
    where
        R: io::Read + io::Seek,
    {
        let record_size = _decode_version3_int4_int8(decoder)?;
        let record_type = CdfInt4::decode(decoder)?;
        if *record_type != 2 {
            return Err(DecodeError::Other(format!(
                "Invalid record_type for GDR - expected 2, received {}",
                *record_type
            )));
        };
        let rvdr_head = {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 {
                None
            } else {
                Some(_v)
            }
        };

        let zvdr_head = {
            // zVDR were introduced in CDF v2.2 and are undefined for earlier versions.
            // if decoder.version < Version::new(2, 2, 0) {
            //     None
            // } else {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 || decoder.version < Version::new(2, 2, 0) {
                None
            } else {
                Some(_v)
            }
        };

        let adr_head = {
            let _v = _decode_version3_int4_int8(decoder)?;
            if *_v == 0 {
                None
            } else {
                Some(_v)
            }
        };

        // eof is undefined for CDF < v2.1
        let eof = {
            let _eof = _decode_version3_int4_int8(decoder)?;
            if decoder.version < Version::new(2, 1, 0) {
                None
            } else {
                Some(_eof)
            }
        };

        let num_rvars = CdfInt4::decode(decoder)?;
        let num_attributes = CdfInt4::decode(decoder)?;
        let max_rvar = CdfInt4::decode(decoder)?;
        let dim_rvar = CdfInt4::decode(decoder)?;
        let num_zvars = CdfInt4::decode(decoder)?;
        let uir_head = _decode_version3_int4_int8(decoder)?;

        let rfu_c = CdfInt4::decode(decoder)?;
        if *rfu_c != 0 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_c read from file - expected 0, received {}",
                *rfu_c
            )));
        }

        let date_last_leapsecond_update = CdfInt4::decode(decoder)?;

        let rfu_e = CdfInt4::decode(decoder)?;
        if *rfu_e != -1 {
            return Err(DecodeError::Other(format!(
                "Invalid rfu_e read from file - expected -1, received {}",
                *rfu_e
            )));
        }

        let mut sizes_rvar = vec![CdfInt4::from(0); *dim_rvar as usize].into_boxed_slice();
        for i in 0..*dim_rvar as usize {
            // If there are rVariables present, read in their dimensions.
            sizes_rvar[i] = CdfInt4::decode(decoder)?;
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
            dim_rvar,
            num_zvars,
            uir_head,
            rfu_c,
            date_last_leapsecond_update,
            rfu_e,
            sizes_rvar,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::cdf;
    use crate::error::CdfError;
    use crate::repr::Endian;
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
            eof: Some(CdfInt8::from(117050)),
            num_rvars: CdfInt4::from(0),
            num_attributes: CdfInt4::from(11),
            max_rvar: CdfInt4::from(-1),
            dim_rvar: CdfInt4::from(0),
            num_zvars: CdfInt4::from(21),
            uir_head: CdfInt8::from(10964),
            rfu_c: CdfInt4::from(0),
            date_last_leapsecond_update: CdfInt4::from(20170101),
            rfu_e: CdfInt4::from(-1),
            sizes_rvar: vec![].into_boxed_slice(),
        };
        let expected2 = GlobalDescriptorRecord {
            record_size: CdfInt8::from(64),
            record_type: CdfInt4::from(2),
            rvdr_head: Some(CdfInt8::from(4405)),
            zvdr_head: None,
            adr_head: Some(CdfInt8::from(376)),
            eof: Some(CdfInt8::from(8420394)),
            num_rvars: CdfInt4::from(15),
            num_attributes: CdfInt4::from(27),
            max_rvar: CdfInt4::from(134639),
            dim_rvar: CdfInt4::from(1),
            num_zvars: CdfInt4::from(0),
            uir_head: CdfInt8::from(0),
            rfu_c: CdfInt4::from(0),
            date_last_leapsecond_update: CdfInt4::from(-1),
            rfu_e: CdfInt4::from(-1),
            sizes_rvar: vec![CdfInt4::from(3)].into_boxed_slice(),
        };
        _ = _gdr_example(file1, expected1)?;
        _ = _gdr_example(file2, expected2)?;
        Ok(())
    }

    fn _gdr_example(filename: &str, exp: GlobalDescriptorRecord) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "data", filename]
            .iter()
            .collect();

        let f = File::open(path_test_file)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader, Endian::Big, None)?;
        let cdf = cdf::Cdf::decode(&mut decoder)?;
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
        assert_eq!(gdr.dim_rvar, exp.dim_rvar);
        assert_eq!(gdr.num_zvars, exp.num_zvars);
        assert_eq!(gdr.uir_head, exp.uir_head);
        assert_eq!(gdr.rfu_c, exp.rfu_c);
        assert_eq!(
            gdr.date_last_leapsecond_update,
            exp.date_last_leapsecond_update
        );
        assert_eq!(gdr.rfu_e, exp.rfu_e);
        assert_eq!(gdr.sizes_rvar.len(), exp.sizes_rvar.len());
        for i in 0..gdr.sizes_rvar.len() {
            assert_eq!(gdr.sizes_rvar[i], exp.sizes_rvar[i]);
        }
        Ok(())
    }
}
