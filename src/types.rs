use crate::error::DecodeError;
use std::{io, mem};

/// Data types supported by the CDF format.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum CdfType {
    /// 1-byte signed integer
    Int1 = 1,
    /// 2-byte signed integer
    Int2 = 2,
    /// 4-byte signed integer
    Int4 = 4,
    /// 8-byte signed integer
    Int8 = 8,
    /// 1-byte unsigned integer
    Uint1 = 11,
    /// 2-byte unsigned integer
    Uint2 = 12,
    /// 4-byte unsigned integer
    Uint4 = 14,
    /// 4-byte single-precision floating-point
    Real4 = 21,
    /// 8-byte double-precision floating-point
    Real8 = 22,
    /// 8-byte double-precision floating-point. Represents the number of milliseconds since epoch
    /// 0000-01-01T00:00:00.000 .
    Epoch = 31,
    /// 2 8-byte double-precision floating-point. Similar to `Epoch` but can store much higher
    /// resolution, down to pico-seconds.
    Epoch16 = 32,
    /// 8-byte signed integer. Nano-seconds from J2000 (2000-01-01T12:00), with leap seconds
    /// included.
    TimeTt2000 = 33,
    /// 1-byte signed integer (equivalent to `Int1`)
    Byte = 41,
    /// 4-byte single-precision floating-point (equivalent to `Real4`)
    Float = 44,
    /// 8-byte double-precision floating-point (equivalent to `Real8`)
    Double = 45,
    /// 1-byte signed character (ASCII)
    Char = 51,
    /// 1-byte unsigned character (ASCII)
    Uchar = 52,
}

macro_rules! impl_cdf_type {
    ($name:ident, $t:ty) => {
        pub struct $name($t);

        impl $name {
            pub const fn size() -> usize {
                mem::size_of::<$t>()
            }

            pub fn from_ne_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$t>::from_ne_bytes(bytes))
            }
            pub fn from_be_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$t>::from_be_bytes(bytes))
            }
            pub fn from_le_bytes(bytes: [u8; Self::size()]) -> Self {
                Self(<$t>::from_le_bytes(bytes))
            }
            pub fn to_ne_bytes(self) -> [u8; Self::size()] {
                <$t>::to_ne_bytes(self.0)
            }
            pub fn to_be_bytes(self) -> [u8; Self::size()] {
                <$t>::to_be_bytes(self.0)
            }
            pub fn to_le_bytes(self) -> [u8; Self::size()] {
                <$t>::to_le_bytes(self.0)
            }
        }

        impl From<$t> for $name {
            fn from(value: $t) -> Self {
                $name(value)
            }
        }
    };
}

impl_cdf_type!(Int1, i8);
impl_cdf_type!(Int2, i16);
impl_cdf_type!(Int4, i32);
impl_cdf_type!(Int8, i64);
impl_cdf_type!(Uint1, u8);
impl_cdf_type!(Uint2, u16);
impl_cdf_type!(Uint4, u32);
impl_cdf_type!(Real4, f32);
impl_cdf_type!(Real8, f64);
impl_cdf_type!(Epoch, f64);
impl_cdf_type!(TimeTt2000, i64);
impl_cdf_type!(Byte, i8);
impl_cdf_type!(Char, i8);
impl_cdf_type!(Uchar, u8);
pub type Float = Real4;
pub type Double = Real8;

pub struct Epoch16(Real8, Real8);

impl Epoch16 {
    pub const fn size() -> usize {
        16
    }
}

//pub fn decode_cdf_type<R>(
//    decoder: &mut Decoder<R>,
//    val_type: CdfType,
//) -> Result<CdfType, DecodeError>
//where
//    R: io::Read,
//{
//    match val_type {
//        CdfType::Int1 => Ok(CdfInt1::decode(decoder)?),
//        CdfType::Int2 => Ok(CdfInt2::decode(decoder)?),
//        CdfType::Int4 => Ok(CdfInt4::decode(decoder)?),
//        CdfType::Int8 => Ok(CdfInt8::decode(decoder)?),
//        CdfType::Uint1 => Ok(CdfUint1::decode(decoder)?),
//        CdfType::Uint2 => Ok(CdfUint2::decode(decoder)?),
//        CdfType::Uint4 => Ok(CdfUint4::decode(decoder)?),
//        CdfType::Real4 => Ok(CdfReal4::decode(decoder)?),
//        CdfType::Real8 => Ok(CdfReal8::decode(decoder)?),
//        CdfType::Epoch => Ok(CdfEpoch::decode(decoder)?),
//        CdfType::Epoch16 => {
//            let v1 = f64::decode(decoder)?;
//            let v2 = f64::decode(decoder)?;
//            Ok((v1, v2))
//        }
//        CdfType::TimeTt2000 => Ok(CdfTimeTt2000::decode(decoder)?),
//        CdfType::Byte => Ok(CdfByte::decode(decoder)?),
//        CdfType::Float => Ok(CdfFloat::decode(decoder)?),
//        CdfType::Double => Ok(CdfDouble::decode(decoder)?),
//        CdfType::Char => Ok(CdfChar::decode(decoder)?),
//        CdfType::Uchar => Ok(CdfUchar::decode(decoder)?),
//        _ => Err(DecodeError::Other("".to_string())),
//    }
//}
//
//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::decode::Decoder;
//    use crate::repr::Encoding;
//
//    #[test]
//    fn test_cdftype_int1() -> Result<(), DecodeError> {
//        let val = 7i8.to_be_bytes();
//        let mut decoder = Decoder {
//            reader: val.as_slice(),
//            encoding: Encoding::Network,
//        };
//        assert_eq!(
//            CdfType::Int1(7i8),
//            decode_cdf_type(&mut decoder, CdfType::Int1(0))?
//        );
//
//        Ok(())
//    }
//}
