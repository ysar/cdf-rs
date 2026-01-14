use std::io;

use crate::error::CdfError;
use crate::repr::{CdfEncoding, CdfVersion, Endian};
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
        num_elements: &CdfInt4,
    ) -> Result<Vec<Self>, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let n = usize::try_from(**num_elements)?;
        let mut result: Vec<Self> = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(Self::decode_be(decoder)?);
        }
        Ok(result)
    }

    /// Decode a sequential collection of this type into a vector using little-endian encoding.
    fn decode_vec_le<R>(
        decoder: &mut Decoder<R>,
        num_elements: &CdfInt4,
    ) -> Result<Vec<Self>, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let n = usize::try_from(**num_elements)?;
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
            context: DecodeContext::default(),
        })
    }
}

/// Stores various contextual values read in the CDF that other records depend on for their decoding.
#[derive(Default)]
pub struct DecodeContext {
    /// The "encoding" of the values in the CDF. This has to be read in or specified for every
    /// CDF file and is contained in the CDR.
    pub encoding: Option<CdfEncoding>,
    /// The endianness of data stored in this CDF.
    pub endianness: Option<Endian>,
    /// CDF version.  This is necessary to include in the decoder since different versions have
    /// different formats.
    pub version: Option<CdfVersion>,
    /// Number of dimensions of rVariables. This is used by the rVDR and is global to the CDF.
    pub num_r_dims: Option<CdfInt4>,
    /// Dimension sizes of rVariables. This is used by the rVDR and is global to the CDF.
    pub size_r_dims: Option<Vec<CdfInt4>>,
    /// Number of dimensions of the zVariable that is currently being read. This is set and used
    /// for the zVDR.
    pub num_z_dims: Option<CdfInt4>,
    /// Dimension sizes of the zVariable that is currently being read. This is set and used for the
    /// zVDR.
    pub size_z_dims: Option<Vec<CdfInt4>>,
    /// Data type of the currently read variable (either rVariable or zVariable)
    pub var_data_type: Option<CdfInt4>,
    /// Number of data of var_data_type in each variable record of the currently read variable (
    /// either rVariable or zVariable)
    pub var_data_len: Option<CdfInt4>,
    /// Number of variable records stored within the current variable values record.
    pub num_records: Option<usize>,
    /// Whether variable records are stored in row-major (true) or column-major (false) format.
    pub row_major: Option<bool>,
}

impl DecodeContext {
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

    /// Sets the encoding for data within this CDF file.
    pub fn set_endianness(&mut self, endianness: Endian) {
        self.endianness = Some(endianness);
    }

    /// Gets the encoding for data within this CDF file.
    /// # Errors
    /// Will raise a [`CdfError`] if the encoding is not yet specified.
    pub fn get_endianness(&self) -> Result<Endian, CdfError> {
        self.endianness.clone().ok_or(CdfError::Decode(
            "No endianness stored in the decoding context.".to_string(),
        ))
    }

    /// Sets the number of dimensions for rVariables within this CDF file.
    pub fn set_num_dimension_rvariable(&mut self, num_dim: CdfInt4) {
        self.num_r_dims = Some(num_dim);
    }

    /// Gets the number of dimensions for rVariables within this CDF file.
    pub fn get_num_dimension_rvariable(&mut self) -> Result<CdfInt4, CdfError> {
        self.num_r_dims.clone().ok_or(CdfError::Decode(
            "No rVariable dimension length stored in decoding context.".to_string(),
        ))
    }

    /// Sets the dimension for rVariables within this CDF file.
    pub fn set_size_dimension_rvariable(&mut self, size_dim: Vec<CdfInt4>) {
        self.size_r_dims = Some(size_dim);
    }

    /// Gets the dimension for rVariables within this CDF file.
    pub fn get_size_dimension_rvariable(&mut self) -> Result<Vec<CdfInt4>, CdfError> {
        self.size_r_dims.clone().ok_or(CdfError::Decode(
            "No rVariable dimensions stored in decoding context.".to_string(),
        ))
    }

    /// Sets the number of dimensions for the active zVariable.
    pub fn set_num_dimension_zvariable(&mut self, num_dim: CdfInt4) {
        self.num_z_dims = Some(num_dim);
    }

    /// Gets the number of dimensions for the active zVariable.
    pub fn get_num_dimension_zvariable(&mut self) -> Result<CdfInt4, CdfError> {
        self.num_z_dims.clone().ok_or(CdfError::Decode(
            "No zVariable dimension length stored in decoding context.".to_string(),
        ))
    }

    /// Sets the dimension for the active zVariable.
    pub fn set_size_dimension_zvariable(&mut self, size_dim: Vec<CdfInt4>) {
        self.size_z_dims = Some(size_dim);
    }

    /// Gets the dimension for the active zVariable.
    pub fn get_size_dimension_zvariable(&mut self) -> Result<Vec<CdfInt4>, CdfError> {
        self.size_z_dims.clone().ok_or(CdfError::Decode(
            "No zVariable dimensions stored in decoding context.".to_string(),
        ))
    }

    /// Sets the dimension for rVariables within this CDF file.
    pub fn set_row_majority(&mut self, row_major: bool) {
        self.row_major = Some(row_major);
    }

    /// Gets the data type of the currently read variable.
    pub fn get_var_data_type(&mut self) -> Result<CdfInt4, CdfError> {
        self.var_data_type.clone().ok_or(CdfError::Decode(
            "No variable type stored in decoding context.".to_string(),
        ))
    }
    /// Sets the data type of the currently read variable.
    pub fn set_var_data_type(&mut self, data_type: CdfInt4) {
        self.var_data_type = Some(data_type);
    }

    /// Gets the number of data_types in each variable record of the currently read variable.
    pub fn get_var_data_len(&mut self) -> Result<CdfInt4, CdfError> {
        self.var_data_len.clone().ok_or(CdfError::Decode(
            "Number of data_type in VR not stored in decoding context.".to_string(),
        ))
    }
    /// Sets the number of data_types in each variable record of the currently read variable.
    pub fn set_var_data_len(&mut self, data_type: CdfInt4) {
        self.var_data_len = Some(data_type);
    }

    /// Gets the number of variable records stored inside the current variable values record.
    pub fn get_num_records(&mut self) -> Result<usize, CdfError> {
        self.num_records.ok_or(CdfError::Decode(
            "Number of records not stored in decoding context.".to_string(),
        ))
    }
    /// Sets the number of variable records stored inside the current variable values record.
    pub fn set_num_records(&mut self, num_records: usize) {
        self.num_records = Some(num_records);
    }

    /// Gets the dimension for rVariables within this CDF file.
    pub fn is_row_major(&mut self) -> Result<bool, CdfError> {
        self.row_major.ok_or(CdfError::Decode(
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
