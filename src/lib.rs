/// Error types for the crate.
pub mod error;

/// Module for traits common to this library, e.g. `Decode` or `Encode`.
pub mod traits;

/// Module containing structs for different kinds of CDF records.
pub mod record;

/// Module associated with the overall structure of a CDF file. Contains the CdfFile struct.
pub mod cdf;
