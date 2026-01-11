use std::io;

use crate::error::CdfError;
use crate::repr::{CdfEncoding, CdfVersion};
use crate::types::{CdfInt4, CdfInt8};

/// Trait for decoding a CDF result from a reader.
pub trait Decodable: Sized {
    /// Decode a value from the input that implements `io::Read` and `io::Seek` using Big-Endian
    /// encoding.
    /// # Errors
    /// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
    fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek;

    /// Decode a value from the input that implements `io::Read` and `io::Seek` using Little-Endian
    /// encoding.
    /// # Errors
    /// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
    fn decode_le<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek;

    /// Decode a sequential collection of this type into a vector using big-endian encoding.
    fn decode_vec_be<R>(
        decoder: &mut Decoder<R>,
        num_elements: CdfInt4,
    ) -> Result<Vec<Self>, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let n = usize::try_from(*num_elements)?;
        let mut result: Vec<Self> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(Self::decode_be(decoder)?);
        }
        Ok(result)
    }

    /// Decode a sequential collection of this type into a vector using little-endian encoding.
    fn decode_vec_le<R>(
        decoder: &mut Decoder<R>,
        num_elements: CdfInt4,
    ) -> Result<Vec<Self>, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let n = usize::try_from(*num_elements)?;
        let mut result: Vec<Self> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(Self::decode_le(decoder)?);
        }
        Ok(result)
    }
}

/// Struct containing the reader and decoding configurations.
pub struct Decoder<R>
where
    R: io::Read + io::Seek,
{
    /// A reader is some object that implements [`io::Read`] and [`io::Seek`].
    pub reader: R,
    /// Context keeps track of values that are needed by other records for decoding.
    pub context: DecodeContext,
}

impl<R> Decoder<R>
where
    R: io::Read + io::Seek,
{
    /// Create a new decoder based on some reader than implements [`io::Read`] and a CDF encoding.
    /// # Errors
    /// Returns a [`CdfError`] if the decoder cannot be constructed.
    pub fn new(reader: R) -> Result<Self, CdfError> {
        Ok(Decoder {
            reader,
            context: DecodeContext::empty(),
        })
    }
}

/// Stores various contextual values read in the CDF that other records depend on for their decoding.
pub struct DecodeContext {
    /// The "encoding" of the values in the CDF. This has to be read in or specified for every
    /// CDF file and is contained in the CDR.
    encoding: Option<CdfEncoding>,
    /// CDF version.  This is necessary to include in the decoder since different versions have
    /// different formats.
    version: Option<CdfVersion>,
    /// Number of dimensions of rVariables. This is used by the RVDR.
    r_num_dims: Option<CdfInt4>,
    /// Whether variable records are stored in row-major (true) or column-major (false) format.
    row_major: Option<bool>,
}

impl DecodeContext {
    /// Create an empty context with nothing specified.
    pub fn empty() -> Self {
        Self {
            encoding: None,
            version: None,
            r_num_dims: None,
            row_major: None,
        }
    }

    /// Sets the CDF version for this context.
    pub fn set_version(&mut self, version: CdfVersion) {
        self.version = Some(version);
    }

    /// Get the CDF version for this context.
    /// # Errors
    /// Will raise a [`CdfError`] if the version is not specified yet.
    pub fn get_version(&self) -> Result<CdfVersion, CdfError> {
        self.version.clone().ok_or(CdfError::Decode(
            "No CDF version stored in the decoding context.".to_string(),
        ))
    }

    /// Sets the encoding for data within this CDF file.
    pub fn set_encoding(&mut self, encoding: CdfEncoding) {
        self.encoding = Some(encoding);
    }

    /// Gets the encoding for data within this CDF file.
    /// # Errors
    /// Will raise a [`CdfError`] if the encoding is not yet specified.
    pub fn get_encoding(&self) -> Result<CdfEncoding, CdfError> {
        self.encoding.clone().ok_or(CdfError::Decode(
            "No CDF encoding stored in the decoding context.".to_string(),
        ))
    }

    /// Sets the dimension for rVariables within this CDF file.
    pub fn set_num_dimension_rvariable(&mut self, num_dim: CdfInt4) {
        self.r_num_dims = Some(num_dim);
    }

    /// Gets the dimension for rVariables within this CDF file.
    pub fn get_num_dimension_rvariable(&mut self) -> Result<CdfInt4, CdfError> {
        self.r_num_dims.clone().ok_or(CdfError::Decode(
            "No rVariable dimension length stored in decoding context.".to_string(),
        ))
    }

    /// Sets the dimension for rVariables within this CDF file.
    pub fn set_row_majority(&mut self, row_major: bool) {
        self.row_major = Some(row_major);
    }

    /// Gets the dimension for rVariables within this CDF file.
    pub fn is_row_major(&mut self) -> Result<bool, CdfError> {
        self.row_major.clone().ok_or(CdfError::Decode(
            "No rVariable dimension length stored in decoding context.".to_string(),
        ))
    }
}

/// CDF versions prior to 3.0 use 4-byte signed integer to store file-offsets pointing to various
/// records.  This was changed to 8-bytes after 3.0.  So, we need to do version-aware decoding.
/// Safely converts [`CdfInt4`] to [`CdfInt8`] after decoding.
/// # Errors
/// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
pub fn decode_version3_int4_int8<R>(decoder: &mut Decoder<R>) -> Result<CdfInt8, CdfError>
where
    R: io::Read + io::Seek,
{
    if decoder.context.get_version()?.major >= 3 {
        CdfInt8::decode_be(decoder)
    } else {
        let s: i32 = CdfInt4::decode_be(decoder)?.into();
        Ok(CdfInt8::from(i64::from(s)))
    }
}
