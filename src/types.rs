/// The CDF format supports different data types like ints and floats of
/// different sizes. This module defines these fundamental types (CdfXXXX) and
/// there conversions from and into byte arrays and native Rust types.
use crate::decode::{Decodable, Decoder};
use crate::error::CdfError;
use std::fmt::{self, Debug, Display, Formatter};
use std::io;
use std::mem;
use std::ops::Deref;

macro_rules! impl_cdf_type {
    ($cdf_type:ident, $rust_type:ty) => {
        #[derive(PartialEq, Clone)]
        #[doc = concat!("CDF-consistent type that is a wrapper around [`", stringify!($rust_type), "`].")]
        pub struct $cdf_type($rust_type);

        impl $cdf_type {
            /// Size of this type in bytes.
            pub const fn size() -> usize {
                mem::size_of::<$rust_type>()
            }

            /// Create an instance from a byte array using big-endian endianness.
            pub fn from_be_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$rust_type>::from_be_bytes(bytes))
            }
            /// Create an instance from a byte array using little-endian endianness.
            pub fn from_le_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$rust_type>::from_le_bytes(bytes))
            }
            /// Convert from this type to a byte array using big-endian endianness.
            pub fn to_be_bytes(self) -> [u8; Self::size()] {
                <$rust_type>::to_be_bytes(self.0)
            }
            /// Convert from this type to a byte array using little-endian endianness.
            pub fn to_le_bytes(self) -> [u8; Self::size()] {
                <$rust_type>::to_le_bytes(self.0)
            }
        }
    };
}
macro_rules! impl_cdf_rust_from {
    ($cdf_type:ident, $rust_type:ty) => {
        impl From<$rust_type> for $cdf_type {
            fn from(value: $rust_type) -> Self {
                $cdf_type(value)
            }
        }

        impl From<$cdf_type> for $rust_type {
            fn from(value: $cdf_type) -> $rust_type {
                value.0
            }
        }
    };
}

macro_rules! impl_cdf_rust_ptr {
    ($cdf_type:ident, $rust_type:ty) => {
        impl AsRef<$rust_type> for $cdf_type {
            fn as_ref(&self) -> &$rust_type {
                &self.0
            }
        }

        impl Deref for $cdf_type {
            type Target = $rust_type;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

macro_rules! impl_cdf_display_debug {
    ($cdf_type:ident) => {
        impl Display for $cdf_type {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{}", self.0)
            }
        }
        impl Debug for $cdf_type {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{}", self.0)
            }
        }
    };
}

// Each CdfType is encoded/decoded in little or big-endian format depending on the type of
// CdfEncoding that is used.
macro_rules! impl_decodable {
    ($cdf_type:ident) => {
        impl Decodable for $cdf_type {
            type Value = Self;

            fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
            where
                R: io::Read + io::Seek,
            {
                let mut buffer = [0u8; <$cdf_type>::size()];

                decoder.reader.read_exact(&mut buffer[..])?;

                Ok($cdf_type::from_be_bytes(buffer))
            }

            fn decode_le<R>(decoder: &mut Decoder<R>) -> Result<Self, CdfError>
            where
                R: io::Read + io::Seek,
            {
                let mut buffer = [0u8; <$cdf_type>::size()];

                decoder.reader.read_exact(&mut buffer[..])?;

                Ok($cdf_type::from_le_bytes(buffer))
            }
        }
    };
}

impl_cdf_type!(CdfInt1, i8);
impl_cdf_type!(CdfInt2, i16);
impl_cdf_type!(CdfInt4, i32);
impl_cdf_type!(CdfInt8, i64);
impl_cdf_type!(CdfUint1, u8);
impl_cdf_type!(CdfUint2, u16);
impl_cdf_type!(CdfUint4, u32);
impl_cdf_type!(CdfReal4, f32);
impl_cdf_type!(CdfReal8, f64);
impl_cdf_type!(CdfEpoch, f64);
impl_cdf_type!(CdfTimeTt2000, i64);
impl_cdf_type!(CdfByte, i8);

impl_cdf_rust_from!(CdfInt1, i8);
impl_cdf_rust_from!(CdfInt2, i16);
impl_cdf_rust_from!(CdfInt4, i32);
impl_cdf_rust_from!(CdfInt8, i64);
impl_cdf_rust_from!(CdfUint1, u8);
impl_cdf_rust_from!(CdfUint2, u16);
impl_cdf_rust_from!(CdfUint4, u32);
impl_cdf_rust_from!(CdfReal4, f32);
impl_cdf_rust_from!(CdfReal8, f64);
impl_cdf_rust_from!(CdfEpoch, f64);
impl_cdf_rust_from!(CdfTimeTt2000, i64);
impl_cdf_rust_from!(CdfByte, i8);

impl_cdf_rust_ptr!(CdfInt1, i8);
impl_cdf_rust_ptr!(CdfInt2, i16);
impl_cdf_rust_ptr!(CdfInt4, i32);
impl_cdf_rust_ptr!(CdfInt8, i64);
impl_cdf_rust_ptr!(CdfUint1, u8);
impl_cdf_rust_ptr!(CdfUint2, u16);
impl_cdf_rust_ptr!(CdfUint4, u32);
impl_cdf_rust_ptr!(CdfReal4, f32);
impl_cdf_rust_ptr!(CdfReal8, f64);
impl_cdf_rust_ptr!(CdfEpoch, f64);
impl_cdf_rust_ptr!(CdfTimeTt2000, i64);
impl_cdf_rust_ptr!(CdfByte, i8);

impl_cdf_display_debug!(CdfInt1);
impl_cdf_display_debug!(CdfInt2);
impl_cdf_display_debug!(CdfInt4);
impl_cdf_display_debug!(CdfInt8);
impl_cdf_display_debug!(CdfUint1);
impl_cdf_display_debug!(CdfUint2);
impl_cdf_display_debug!(CdfUint4);
impl_cdf_display_debug!(CdfReal4);
impl_cdf_display_debug!(CdfReal8);
impl_cdf_display_debug!(CdfEpoch);
impl_cdf_display_debug!(CdfTimeTt2000);
impl_cdf_display_debug!(CdfByte);

impl_decodable!(CdfInt1);
impl_decodable!(CdfInt2);
impl_decodable!(CdfInt4);
impl_decodable!(CdfInt8);
impl_decodable!(CdfUint1);
impl_decodable!(CdfUint2);
impl_decodable!(CdfUint4);
impl_decodable!(CdfReal4);
impl_decodable!(CdfReal8);
impl_decodable!(CdfEpoch);
impl_decodable!(CdfTimeTt2000);
impl_decodable!(CdfByte);

/// CDF-consistent type that is a wrapper around [`char`] with checks to ensure that it is ASCII.
/// This the unsigned version with valid values of 0-127 in ASCII and 128-255 in extended ASCII.
/// It is not recommended to use this type for strings stored in the CDF file anymore, since
/// v3.8.1 allows for UTF-8 encoding.
/// This type is equivalent to [`CdfUchar`].
#[derive(PartialEq, Clone)]
pub struct CdfChar(char);

impl CdfChar {
    /// Size of this type in bytes. A CdfChar contains ASCII and is 1 byte long.
    pub const fn size() -> usize {
        1
    }

    /// Create an instance from a byte array using big-endian endianness.
    pub fn from_be_bytes(bytes: [u8; 1]) -> Self {
        Self(char::from(u8::from_be_bytes(bytes)))
    }

    /// Create an instance from a byte array using little-endian endianness.
    pub fn from_le_bytes(bytes: [u8; 1]) -> Self {
        Self(char::from(u8::from_le_bytes(bytes)))
    }

    /// Convert from this type to a byte array using big-endian endianness.
    pub fn to_be_bytes(self) -> [u8; 1] {
        u8::to_be_bytes(self.0 as u8) // We are sure that CdfChar is ASCII.
    }
    /// Convert from this type to a byte array using little-endian endianness.
    pub fn to_le_bytes(self) -> [u8; 1] {
        u8::to_le_bytes(self.0 as u8) // We are sure that CdfChar is ASCII.
    }
}

// For CdfChar only, we will use try_from because `char` may not be ASCII.
impl TryFrom<char> for CdfChar {
    type Error = CdfError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let repr = u8::try_from(value).map_err(|_| {
            CdfError::Decode(format!("Unable to convert unicode {value} into ASCII."))
        })?;
        Ok(CdfChar(repr as char))
    }
}

impl From<CdfChar> for char {
    fn from(value: CdfChar) -> char {
        value.0
    }
}

impl_cdf_rust_ptr!(CdfChar, char);
impl_cdf_display_debug!(CdfChar);
impl_decodable!(CdfChar);

/// Alias for [`CdfUchar`].  Using either of these types for creating new CDF files is not
/// recommended and the new approach using [`CdfString`] is preferred due to UTF-8 support.
pub type CdfUchar = CdfChar;

#[doc = concat!("CDF-consistent type that is a wrapper around `([`CdfReal8`], [`CdfReal8`])`.")]
pub struct CdfEpoch16(CdfReal8, CdfReal8);

impl CdfEpoch16 {
    /// Size of this type in bytes.
    pub const fn size() -> usize {
        16
    }
    /// Create an instance from a byte array using big-endian endianness.
    pub fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(
            CdfReal8::from_be_bytes(bytes[0..8].try_into().unwrap()),
            CdfReal8::from_be_bytes(bytes[8..16].try_into().unwrap()),
        )
    }
    /// Create an instance from a byte array using little-endian endianness.
    pub fn from_le_bytes(bytes: [u8; 16]) -> Self {
        Self(
            CdfReal8::from_le_bytes(bytes[0..8].try_into().unwrap()),
            CdfReal8::from_le_bytes(bytes[8..16].try_into().unwrap()),
        )
    }

    // CAUTION: It is unclear how (Real8, Real8) values are stored. Is the
    // endianness only relevant within each field or on both fields as a whole?

    /// Convert from this type to a byte array using big-endian endianness.
    #[rustfmt::skip]
    pub fn to_be_bytes(self) -> [u8; 16] {
        let r1 = self.0.to_be_bytes();
        let r2 = self.1.to_be_bytes();
        [
            r1[0], r1[1], r1[2], r1[3], r1[4], r1[5], r1[6], r1[7],
            r2[0], r2[1], r2[2], r2[3], r2[4], r2[5], r2[6], r2[7],
        ]
    }
    /// Convert from this type to a byte array using little-endian endianness.
    #[rustfmt::skip]
    pub fn to_le_bytes(self) -> [u8; 16] {
        let r1 = self.0.to_le_bytes();
        let r2 = self.1.to_le_bytes();
        [
            r1[0], r1[1], r1[2], r1[3], r1[4], r1[5], r1[6], r1[7],
            r2[0], r2[1], r2[2], r2[3], r2[4], r2[5], r2[6], r2[7],
        ]
    }
}

impl Debug for CdfEpoch16 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.0, self.1)
    }
}
impl_decodable!(CdfEpoch16);

/// CDF-consistent type that is a wrapper around [`String`]. This is not defined in the CDF
/// specification but is useful for string operations.
pub struct CdfString(String);

impl CdfString {
    /// Create a CDF-compatible string using a slice of CdfChars. This method is provided to read
    /// legacy CDF files that store strings as a collection of [`CdfUchar`] or [`CdfChar`].
    pub fn from_slice_chars(chars: &[CdfChar]) -> Self {
        CdfString(chars.iter().map(|c| c.0).collect())
    }

    /// Decode a collection of bytes of length `num_bytes` into a [`CdfString`]
    pub fn decode_string_from_numbytes<R>(
        decoder: &mut Decoder<R>,
        num_bytes: usize,
    ) -> Result<Self, CdfError>
    where
        R: io::Read + io::Seek,
    {
        let mut buffer = vec![0u8; num_bytes];
        _ = decoder.reader.read_exact(&mut buffer);
        Ok(
            String::from_utf8(buffer.into_iter().take_while(|c| *c != 0).collect())
                .map_err(|e| CdfError::Decode(format!("Error decoding string - {e}")))?
                .into(),
        )
    }
}

impl_cdf_rust_from!(CdfString, String);
impl_cdf_rust_ptr!(CdfString, String);
impl_cdf_display_debug!(CdfString);

// This enum stores the various allowed CDF types as defined in the specification.  The double
// indirection is ugly but it is necessary for generalizing various CDF records.  The alternative
// would have been to use a trait (say `CdfType`) and using dynamic dispatch, which may be less
// performant. Even if I used Box<dyn>, it would introduce a layer of indirection. So, for now,
// let's try this way.
/// The enum wraps the more primitive CDF types into one type for use with various records which
/// contain a mixture of different primitive CDF types.
#[repr(i32)]
#[derive(Debug)]
pub enum CdfType {
    /// Wraps [`CdfInt1`].
    Int1(CdfInt1) = 1,
    /// Wraps [`CdfInt2`].
    Int2(CdfInt2) = 2,
    /// Wraps [`CdfInt4`].
    Int4(CdfInt4) = 4,
    /// Wraps [`CdfInt8`].
    Int8(CdfInt8) = 8,
    /// Wraps [`CdfUint1`].
    Uint1(CdfUint1) = 11,
    /// Wraps [`CdfUint2`].
    Uint2(CdfUint2) = 12,
    /// Wraps [`CdfUint4`].
    Uint4(CdfUint4) = 14,
    /// Wraps [`CdfReal4`].
    Real4(CdfReal4) = 21,
    /// Wraps [`CdfReal8`].
    Real8(CdfReal8) = 22,
    /// Wraps [`CdfEpoch`].
    Epoch(CdfEpoch) = 31,
    /// Wraps [`CdfEpoch16`].
    Epoch16(CdfEpoch16) = 32,
    /// Wraps [`CdfTimeTt2000`].
    TimeTt2000(CdfTimeTt2000) = 33,
    /// Wraps [`CdfByte`].
    Byte(CdfByte) = 41,
    /// Wraps [`CdfChar`].
    Char(CdfChar) = 51,
    /// Wraps [`CdfUchar`].
    Uchar(CdfUchar) = 52,
}

/// Decodes any CDF data type assuming Big-Endian encoding, given its numeric identifier, as defined
/// in Table 5.9 in the CDF specification.
/// # Errors
/// Returns a [`CdfError::Decode`] if the decoding fails for any reason.
pub fn decode_cdf_type_be<R>(decoder: &mut Decoder<R>, data_type: i32) -> Result<CdfType, CdfError>
where
    R: io::Read + io::Seek,
{
    match data_type {
        1 => Ok(CdfType::Int1(CdfInt1::decode_be(decoder)?)),
        2 => Ok(CdfType::Int2(CdfInt2::decode_be(decoder)?)),
        4 => Ok(CdfType::Int4(CdfInt4::decode_be(decoder)?)),
        8 => Ok(CdfType::Int8(CdfInt8::decode_be(decoder)?)),
        11 => Ok(CdfType::Uint1(CdfUint1::decode_be(decoder)?)),
        12 => Ok(CdfType::Uint2(CdfUint2::decode_be(decoder)?)),
        14 => Ok(CdfType::Uint4(CdfUint4::decode_be(decoder)?)),
        21 => Ok(CdfType::Real4(CdfReal4::decode_be(decoder)?)),
        22 => Ok(CdfType::Real8(CdfReal8::decode_be(decoder)?)),
        31 => Ok(CdfType::Epoch(CdfEpoch::decode_be(decoder)?)),
        32 => Ok(CdfType::Epoch16(CdfEpoch16::decode_be(decoder)?)),
        33 => Ok(CdfType::TimeTt2000(CdfTimeTt2000::decode_be(decoder)?)),
        41 => Ok(CdfType::Byte(CdfByte::decode_be(decoder)?)),
        44 => Ok(CdfType::Real4(CdfReal4::decode_be(decoder)?)),
        45 => Ok(CdfType::Real8(CdfReal8::decode_be(decoder)?)),
        51 => Ok(CdfType::Char(CdfChar::decode_be(decoder)?)),
        52 => Ok(CdfType::Uchar(CdfUchar::decode_be(decoder)?)),
        e => Err(CdfError::Decode(format!(
            "Invalid CDF data_type received - {}",
            e
        ))),
    }
}

/// Decodes any CDF data type assuming Little-Endian encoding, given its numeric identifier, as defined
/// in Table 5.9 in the CDF specification.
/// # Errors
/// Returns a [`DecodeError`] if the decoding fails for any reason.
pub fn decode_cdf_type_le<R>(decoder: &mut Decoder<R>, data_type: i32) -> Result<CdfType, CdfError>
where
    R: io::Read + io::Seek,
{
    match data_type {
        1 => Ok(CdfType::Int1(CdfInt1::decode_le(decoder)?)),
        2 => Ok(CdfType::Int2(CdfInt2::decode_le(decoder)?)),
        4 => Ok(CdfType::Int4(CdfInt4::decode_le(decoder)?)),
        8 => Ok(CdfType::Int8(CdfInt8::decode_le(decoder)?)),
        11 => Ok(CdfType::Uint1(CdfUint1::decode_le(decoder)?)),
        12 => Ok(CdfType::Uint2(CdfUint2::decode_le(decoder)?)),
        14 => Ok(CdfType::Uint4(CdfUint4::decode_le(decoder)?)),
        21 => Ok(CdfType::Real4(CdfReal4::decode_le(decoder)?)),
        22 => Ok(CdfType::Real8(CdfReal8::decode_le(decoder)?)),
        31 => Ok(CdfType::Epoch(CdfEpoch::decode_le(decoder)?)),
        32 => Ok(CdfType::Epoch16(CdfEpoch16::decode_le(decoder)?)),
        33 => Ok(CdfType::TimeTt2000(CdfTimeTt2000::decode_le(decoder)?)),
        41 => Ok(CdfType::Byte(CdfByte::decode_le(decoder)?)),
        44 => Ok(CdfType::Real4(CdfReal4::decode_le(decoder)?)),
        45 => Ok(CdfType::Real8(CdfReal8::decode_le(decoder)?)),
        51 => Ok(CdfType::Char(CdfChar::decode_le(decoder)?)),
        52 => Ok(CdfType::Uchar(CdfUchar::decode_le(decoder)?)),
        e => Err(CdfError::Decode(format!(
            "Invalid CDF data_type received - {}",
            e
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::Decoder;
    use crate::error::CdfError;
    use paste::paste;

    macro_rules! test_type {
        ($t1:ty, $t2:ty, $val:literal) => {
            paste! {
                #[test]
                fn [< test_convert_ $t1:lower _ $t2 >]() {
                    let x: $t2 = $val;
                    let y: $t1 = x.into();
                    assert_eq!(x, y.into());
                }

                #[test]
                fn [< test_decode_ $t1:lower _ $t2 >]() -> Result<(), CdfError> {
                    let x: $t2 = $val;
                    let y = x.to_be_bytes();
                    let mut decoder = Decoder::new(io::Cursor::new(y.as_slice()))?;
                    assert_eq!($t1(x), $t1::decode_be(&mut decoder)?);

                    Ok(())
                }
            }
        };
    }

    test_type!(CdfInt1, i8, -7);
    test_type!(CdfInt2, i16, -7);
    test_type!(CdfInt4, i32, -7);
    test_type!(CdfInt8, i64, -7);
    test_type!(CdfByte, i8, -7);
    test_type!(CdfTimeTt2000, i64, -7);
    test_type!(CdfUint1, u8, 7);
    test_type!(CdfUint2, u16, 7);
    test_type!(CdfUint4, u32, 7);
    test_type!(CdfReal4, f32, -7.0);
    test_type!(CdfReal8, f64, -7.0);

    #[test]
    fn test_convert_cdfchar_char() {
        let x: char = 'a'; // ASCII
        let y: CdfChar = x.try_into().unwrap();
        assert_eq!(x, y.into());

        let x: char = 'ñ'; // Extended ASCII
        let y: CdfChar = x.try_into().unwrap();
        assert_eq!(x, y.into());

        let x: char = 'Ā'; // Valid Unicode but not ASCII.
        let y: Result<CdfChar, CdfError> = x.try_into();
        assert!(y.is_err());
    }

    #[test]
    fn test_decode_cdfchar_char() -> Result<(), CdfError> {
        let x: char = 'a';
        let y = (x as u8).to_be_bytes();
        let mut decoder = Decoder::new(io::Cursor::new(y.as_slice()))?;
        assert_eq!(CdfChar(x), CdfChar::decode_be(&mut decoder)?);

        let x: char = 'ñ';
        let y = (x as u8).to_be_bytes();
        let mut decoder = Decoder::new(io::Cursor::new(y.as_slice()))?;
        assert_eq!(CdfChar(x), CdfChar::decode_be(&mut decoder)?);

        Ok(())
    }

    // test_float!(CdfEpoch, f64);
}
