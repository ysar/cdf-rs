/// The CDF format supports different data types like ints and floats of
/// different sizes. This module defines these fundamental types (CdfXXXX) and
/// there conversions from and into byte arrays and native Rust types.
use crate::decode::{Decodable, Decoder};
use crate::error::DecodeError;
use std::fmt::{self, Debug, Display, Formatter};
use std::io;
use std::mem;
use std::ops::Deref;

macro_rules! impl_cdf_type {
    ($cdf_type:ident, $rust_type:ty) => {
        #[derive(PartialEq, Clone)]
        pub struct $cdf_type($rust_type);

        impl $cdf_type {
            pub const fn size() -> usize {
                mem::size_of::<$rust_type>()
            }

            pub fn from_ne_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$rust_type>::from_ne_bytes(bytes))
            }
            pub fn from_be_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$rust_type>::from_be_bytes(bytes))
            }
            pub fn from_le_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$rust_type>::from_le_bytes(bytes))
            }
            pub fn to_ne_bytes(self) -> [u8; Self::size()] {
                <$rust_type>::to_ne_bytes(self.0)
            }
            pub fn to_be_bytes(self) -> [u8; Self::size()] {
                <$rust_type>::to_be_bytes(self.0)
            }
            pub fn to_le_bytes(self) -> [u8; Self::size()] {
                <$rust_type>::to_le_bytes(self.0)
            }
        }

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
impl_cdf_type!(CdfChar, i8); // Would be good to store chars here instead of ints.
impl_cdf_type!(CdfUchar, u8);
// pub type CdfFloat = CdfReal4;
// pub type CdfDouble = CdfReal8;

pub struct CdfEpoch16(CdfReal8, CdfReal8);

impl CdfEpoch16 {
    pub const fn size() -> usize {
        16
    }
    pub fn from_ne_bytes(bytes: [u8; 16]) -> Self {
        Self(
            CdfReal8::from_ne_bytes(bytes[0..8].try_into().unwrap()),
            CdfReal8::from_ne_bytes(bytes[8..16].try_into().unwrap()),
        )
    }
    pub fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(
            CdfReal8::from_be_bytes(bytes[0..8].try_into().unwrap()),
            CdfReal8::from_be_bytes(bytes[8..16].try_into().unwrap()),
        )
    }
    pub fn from_le_bytes(bytes: [u8; 16]) -> Self {
        Self(
            CdfReal8::from_le_bytes(bytes[0..8].try_into().unwrap()),
            CdfReal8::from_le_bytes(bytes[8..16].try_into().unwrap()),
        )
    }
    #[rustfmt::skip]
    pub fn to_ne_bytes(self) -> [u8; 16] {
        let r1 = self.0.to_ne_bytes();
        let r2 = self.1.to_ne_bytes();
        [
            r1[0], r1[1], r1[2], r1[3], r1[4], r1[5], r1[6], r1[7],
            r2[0], r2[1], r2[2], r2[3], r2[4], r2[5], r2[6], r2[7],
        ]
    }
    #[rustfmt::skip]
    pub fn to_be_bytes(self) -> [u8; 16] {
        let r1 = self.0.to_be_bytes();
        let r2 = self.1.to_be_bytes();
        [
            r1[0], r1[1], r1[2], r1[3], r1[4], r1[5], r1[6], r1[7],
            r2[0], r2[1], r2[2], r2[3], r2[4], r2[5], r2[6], r2[7],
        ]
    }
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

// Each CdfType is encoded/decoded in little or big-endian format depending on the type of
// CdfEncoding that is used.

macro_rules! impl_decodable {
    ($($cdf_type:ident), *) => {
        $(
            impl Decodable for $cdf_type {
                type Value = Self;

                fn decode_be<R>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError>
                where
                    R: io::Read + io::Seek
                {
                    let mut buffer = [0u8; <$cdf_type>::size()];

                    decoder
                        .reader
                        .read_exact(&mut buffer[..])?;
                        // .map_err(|err| DecodeError(format!("{err}")))?;

                    Ok($cdf_type::from_be_bytes(buffer))
                }

                fn decode_le<R>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError>
                where
                    R: io::Read + io::Seek
                {
                    let mut buffer = [0u8; <$cdf_type>::size()];

                    decoder
                        .reader
                        .read_exact(&mut buffer[..])?;
                        // .map_err(|err| DecodeError(format!("{err}")))?;

                    Ok($cdf_type::from_le_bytes(buffer))
                }
            }
        )*
    }
}

impl_decodable!(CdfUint1, CdfUint2, CdfUint4);
impl_decodable!(CdfInt1, CdfInt2, CdfInt4, CdfInt8);
impl_decodable!(CdfTimeTt2000, CdfByte, CdfChar, CdfUchar);
impl_decodable!(CdfReal4, CdfReal8);
impl_decodable!(CdfEpoch, CdfEpoch16);

// This enum stores the various allowed CDF types as defined in the specification.  The double
// indirection is ugly but it is necessary for generalizing various CDF records.  The alternative
// would have been to use a trait (say `CdfType`) and using dynamic dispatch, which may be less
// performant. Even if I used Box<dyn>, it would introduce a layer of indirection. So, for now,
// let's try this way.
/// The enum wrapper the more primitive CDF types into one type for use with various records which
/// contain a mixture of different primitive CDF types.
#[repr(i32)]
#[derive(Debug)]
pub enum CdfType {
    Int1(CdfInt1) = 1,
    Int2(CdfInt2) = 2,
    Int4(CdfInt4) = 4,
    Int8(CdfInt8) = 8,
    Uint1(CdfUint1) = 11,
    Uint2(CdfUint2) = 12,
    Uint4(CdfUint4) = 14,
    Real4(CdfReal4) = 21,
    Real8(CdfReal8) = 22,
    Epoch(CdfEpoch) = 31,
    Epoch16(CdfEpoch16) = 32,
    TimeTt2000(CdfTimeTt2000) = 33,
    Byte(CdfByte) = 41,
    Char(CdfChar) = 51,
    Uchar(CdfUchar) = 52,
}

/// Decodes any CDF data type assuming Big-Endian encoding, given its numeric identifier, as defined
/// in Table 5.9 in the CDF specification.
pub fn decode_cdf_type_be<R>(
    decoder: &mut Decoder<R>,
    data_type: i32,
) -> Result<CdfType, DecodeError>
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
        e => Err(DecodeError(format!(
            "Invalid CDF data_type received - {}",
            e
        ))),
    }
}

/// Decodes any CDF data type assuming Little-Endian encoding, given its numeric identifier, as defined
/// in Table 5.9 in the CDF specification.
pub fn decode_cdf_type_le<R>(
    decoder: &mut Decoder<R>,
    data_type: i32,
) -> Result<CdfType, DecodeError>
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
        e => Err(DecodeError(format!(
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
    test_type!(CdfChar, i8, -7);
    test_type!(CdfTimeTt2000, i64, -7);
    test_type!(CdfUint1, u8, 7);
    test_type!(CdfUint2, u16, 7);
    test_type!(CdfUint4, u32, 7);
    test_type!(CdfUchar, u8, 7);
    test_type!(CdfReal4, f32, -7.0);
    test_type!(CdfReal8, f64, -7.0);
    // test_float!(CdfEpoch, f64);
}
