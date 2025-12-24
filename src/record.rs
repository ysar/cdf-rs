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
#[derive(Debug)]
pub enum InternalRecord {
    /// CDF Descriptor Record
    CDR(cdr::CdfDescriptorRecord) = 1,
    /// Global Descriptor Record
    GDR(gdr::GlobalDescriptorRecord) = 2,
}
