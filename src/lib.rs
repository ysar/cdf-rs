/// Error types for the crate.
pub mod error;

/// Module for traits common to this library, e.g. `Decode` or `Encode`.
///
/// pub mod traits;
pub mod decode;
pub mod repr;
pub mod types;

/// Module containing structs for different kinds of CDF records.
pub mod record;
