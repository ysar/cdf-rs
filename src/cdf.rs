use std::fs::File;
use std::io::{self, BufReader};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::decode::{Decodable, Decoder};
use crate::error::CdfError;
use crate::record::adr::AttributeDescriptorRecord;
use crate::record::cdr::CdfDescriptorRecord;
use crate::record::collection::get_record_vec;
use crate::record::gdr::GlobalDescriptorRecord;
use crate::record::rvdr::RVariableDescriptorRecord;
use crate::record::uir::UnusedInternalRecord;
use crate::record::zvdr::ZVariableDescriptorRecord;
use crate::repr::CdfVersion;
use crate::types::CdfUint4;

/// General struct to hold the contents of the CDF file.
// #[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Cdf {
    /// CDF Version
    pub version: CdfVersion,
    /// Whether this CDF file is compressed.
    pub is_compressed: bool,
    /// Contents of the CDF Descriptor Record.
    pub cdr: CdfDescriptorRecord,
    /// Contents of the Global Descriptor Record.
    pub gdr: GlobalDescriptorRecord,
}

impl Cdf {
    /// Decode or deserialize a CDF file. This reads in all contents of the CDF file and stores each
    /// record as per the CDF heirarchy.
    pub fn read<P: AsRef<std::path::Path>>(file_path: P) -> Result<Self, CdfError> {
        let f = File::open(file_path)?;
        let reader = BufReader::new(f);
        let mut decoder = Decoder::new(reader)?;

        // Read in the magic numbers and store that in the decoder for later use.
        let (version, is_compressed) = Cdf::read_magic_numbers(&mut decoder)?;
        decoder.context.version = Some(version.clone());

        // After this, read in this CDR and GDR
        let cdr = CdfDescriptorRecord::decode_be(&mut decoder)?;
        let gdr = GlobalDescriptorRecord::decode_be(&mut decoder)?;

        // Now, read in all the  RVDRs, ZVDRs, ADRs, and UIRs.
        let rvdr_vec = match &gdr.rvdr_head {
            Some(head) => get_record_vec::<_, RVariableDescriptorRecord>(&mut decoder, head)?,
            None => vec![],
        };

        let zvdr_vec = match &gdr.zvdr_head {
            Some(head) => get_record_vec::<_, ZVariableDescriptorRecord>(&mut decoder, head)?,
            None => vec![],
        };

        let adr_vec = match &gdr.adr_head {
            Some(head) => get_record_vec::<_, AttributeDescriptorRecord>(&mut decoder, head)?,
            None => vec![],
        };

        let uir_vec = match &gdr.uir_head {
            Some(head) => get_record_vec::<_, UnusedInternalRecord>(&mut decoder, head)?,
            None => vec![],
        };

        Ok(Cdf {
            version,
            is_compressed,
            cdr,
            gdr,
        })
    }

    /// Read the magic numbers at the beginning of the CDF file and return the version and whether
    /// this CDF file is compressed.
    pub fn read_magic_numbers<R>(decoder: &mut Decoder<R>) -> Result<(CdfVersion, bool), CdfError>
    where
        R: io::Read + io::Seek,
    {
        // Decode the magic numbers.  The first number is not that important as it seems.
        let m1 = CdfUint4::decode_be(decoder)?;
        let m2 = CdfUint4::decode_be(decoder)?;

        // This is mostly a hack to get a hint of the CDF version. We read in the actual version
        // properly in the CDR. We need to know before reading the CDR if the CDF is >= v3.0 or
        // not.
        let version = match m1.into() {
            0xcdf30001 => CdfVersion::new(3, 0, 0),
            0xcdf26002 => CdfVersion::new(2, 6, 0),
            0x0000ffff => CdfVersion::new(2, 0, 0),
            v => return Err(CdfError::Decode(format!("Invalid magic number - {v}"))),
        };

        let is_compressed: bool = match m2.into() {
            0x0000ffffu32 => false,
            0xcccc0001u32 => true,
            v => return Err(CdfError::Decode(format!("Invalid magic number - {v}"))),
        };
        Ok((version, is_compressed))
    }
}

#[cfg(test)]
mod tests {

    use crate::error::CdfError;
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_read_cdf() -> Result<(), CdfError> {
        let file1 = "test_alltypes.cdf";
        let file2 = "ulysses.cdf";

        _cdf_example(file1)?;
        _cdf_example(file2)?;
        Ok(())
    }

    fn _cdf_example(filename: &str) -> Result<(), CdfError> {
        let path_test_file: PathBuf = [env!("CARGO_MANIFEST_DIR"), "examples", "data", filename]
            .iter()
            .collect();

        let cdf = Cdf::read(path_test_file)?;
        dbg!(cdf);
        Ok(())
    }
}
