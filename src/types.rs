use crate::decode::{Decode, Decoder};
use crate::error::DecodeError;
use std::io;

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

pub struct Int1(i8);
pub struct Int2(i16);
pub struct Int4(i32);
pub struct Int8(i64);
pub struct Uint1(u8);
pub struct Uint2(u16);
pub struct Uint4(u32);
pub struct Real4(f32);
pub struct Real8(f64);
pub struct Epoch(f64);
pub struct Epoch16(f64, f64);
pub struct TimeTt2000(i64);
pub struct Byte(i8);
pub struct Char(i8);
pub struct Uchar(u8);
pub type Float = Real4;
pub type Double = Real8;

macro_rules! impl_size_cdf_types {
    ($($name:ident, $size:literal), *) => {
        $( impl $name {
            pub const fn size() -> usize { $size }
        }
        )*
    };
}

impl_size_cdf_types!(Int1, 1, Int2, 2, Int4, 4, Int8, 8);
impl_size_cdf_types!(Uint1, 1, Uint2, 2, Uint4, 4);
impl_size_cdf_types!(Real4, 4, Real8, 8);
impl_size_cdf_types!(Epoch, 8, Epoch16, 16);
impl_size_cdf_types!(TimeTt2000, 8, Byte, 1, Char, 1, Uchar, 1);


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
