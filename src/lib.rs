//! Library for reading and writing to files stored in NASA's Common Data Format (CDF).

// #![warn(missing_docs)]

/// Module containing error definitions.
pub mod error;

/// General definitions for structures and traits for decoding CDF data.
pub mod decode;

/// Representation for different CDF types.
pub mod repr;

/// The CDF format contains fundamental types that are common to CDF files across different
/// architectures.
pub mod types;

//pub mod record;
