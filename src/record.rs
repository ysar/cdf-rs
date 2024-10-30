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


pub enum InternalRecord {
    /// CDF Descriptor Record 
    CDR(cdr::CdfDescriptorRecord) = 1
}

impl<R> Decode<R> for InternalRecord {
    
    type Output = InternalRecord;

    fn decode(reader: R) -> Result<Self::Output, crate::error::DecodeError> {
        
        /// The decoder has to figure out what kind of record it is first. Note that CDF is a 
        /// self-describing format, so this is possible.

    }
}
