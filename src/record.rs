use std::io;

use crate::traits::Decode;

pub mod adr;
pub mod agredr;
pub mod azedr;
pub mod ccr;
pub mod cdr;
pub mod cpr;
pub mod cvvr;
pub mod gdr;
pub mod rvdr;
pub mod spr;
pub mod uir;
pub mod vvr;
pub mod vxr;
pub mod zvdr;

/// The kind of internal records that can be stored in a CDF. Each kind wraps a struct containing
/// the associated data inside.
#[repr(i32)]
pub enum InternalRecordType {
    /// CDF Descriptor Record
    CDR(cdr::CdfDescriptorRecord) = 1,
}

/// Stores different kinds of CDF internal records.
pub struct InternalRecord {
    /// The size of the internal record. CDF stores this as i64, but we will use u64.
    record_size: u64,
    record: InternalRecordType,
}

impl<R> Decode<R> for InternalRecord
where
    R: io::Read,
{
    type Output = InternalRecord;

    fn decode(reader: R) -> Result<Self::Output, crate::error::DecodeError> {
        // The decoder has to figure out what kind of record it is first. Note that CDF is a
        // self-describing format, so this is possible.
        //let record_size = i64::decode(reader)?;
        //let record_type = i32::decode(reader)?;
        //
        //match record_type {
        //    InternalRecordType::CDR(_) =>
        //}
        todo!();
    }
}
