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

macro_rules! impl_getter {
    ($field:ident, $type:ty) => {
        #[doc = concat!(
            "Getter method for `", stringify!($field), "` field inside a [`DecodeContext`].",
            " This handles the case of missings fields (None) and will raise a `CdfError::Decode`.",
            " Setters are not implemented since fields are public anyway."
        )]
        pub fn $field(&self) -> Result<$type, CdfError> {
            self.$field.clone().ok_or(CdfError::Decode(format!(
                "Missing {} in decoding context.",
                stringify!($field),
            )))
        }
    }
}
impl DecodeContext {
    impl_getter!(encoding, CdfEncoding);
    impl_getter!(endianness, Endian);
    impl_getter!(version, CdfVersion);
    impl_getter!(num_r_dims, CdfInt4);
    impl_getter!(size_r_dims, Vec<CdfInt4>);
    impl_getter!(num_z_dims, CdfInt4);
    impl_getter!(size_z_dims, Vec<CdfInt4>);
    impl_getter!(var_data_type, CdfInt4);
    impl_getter!(var_data_len, CdfInt4);
    impl_getter!(num_records, usize);
    impl_getter!(row_major, bool);
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
    if decoder.context.version()?.major >= 3 {
        CdfInt8::decode_be(decoder)
    } else {
        let s: i32 = CdfInt4::decode_be(decoder)?.into();
        Ok(CdfInt8::from(i64::from(s)))
    }
}
