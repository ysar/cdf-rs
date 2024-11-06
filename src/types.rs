use crate::decode::{Decode, Decoder};
use crate::error::DecodeError;
use std::io;

/// Data types supported by the CDF format.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum CdfType {
    /// 1-byte signed integer
    Int1(i8) = 1,
    /// 2-byte signed integer
    Int2(i16) = 2,
    /// 4-byte signed integer
    Int4(i32) = 4,
    /// 8-byte signed integer
    Int8(i64) = 8,
    /// 1-byte unsigned integer
    Uint1(u8) = 11,
    /// 2-byte unsigned integer
    Uint2(u16) = 12,
    /// 4-byte unsigned integer
    Uint4(u32) = 14,
    /// 4-byte single-precision floating-point
    Real4(f32) = 21,
    /// 8-byte double-precision floating-point
    Real8(f64) = 22,
    /// 8-byte double-precision floating-point. Represents the number of milliseconds since epoch
    /// 0000-01-01T00:00:00.000 .
    Epoch(f64) = 31,
    /// 2 8-byte double-precision floating-point. Similar to `Epoch` but can store much higher
    /// resolution, down to pico-seconds.
    Epoch16(f64, f64) = 32,
    /// 8-byte signed integer. Nano-seconds from J2000 (2000-01-01T12:00), with leap seconds
    /// included.
    TimeTt2000(i64) = 33,
    /// 1-byte signed integer (equivalent to `Int1`)
    Byte(i8) = 41,
    /// 4-byte single-precision floating-point (equivalent to `Real4`)
    Float(f32) = 44,
    /// 8-byte double-precision floating-point (equivalent to `Real8`)
    Double(f64) = 45,
    /// 1-byte signed character (ASCII)
    Char(char) = 51,
    /// 1-byte unsigned character (ASCII)
    Uchar(char) = 52,
}

pub fn decode_cdf_type<R>(
        decoder: &mut Decoder<R>,
        val_type: CdfType,
    ) -> Result<CdfType, DecodeError>
    where
        R: io::Read,
{
    match val_type {
        CdfType::Int1(_) => Ok(CdfType::Int1(i8::decode(decoder)?)),
        CdfType::Int2(_) => Ok(CdfType::Int2(i16::decode(decoder)?)),
        CdfType::Int4(_) => Ok(CdfType::Int4(i32::decode(decoder)?)),
        CdfType::Int8(_) => Ok(CdfType::Int8(i64::decode(decoder)?)),
        CdfType::Uint1(_) => Ok(CdfType::Uint1(u8::decode(decoder)?)),
        CdfType::Uint2(_) => Ok(CdfType::Uint2(u16::decode(decoder)?)),
        CdfType::Uint4(_) => Ok(CdfType::Uint4(u32::decode(decoder)?)),
        CdfType::Real4(_) => Ok(CdfType::Real4(f32::decode(decoder)?)),
        CdfType::Real8(_) => Ok(CdfType::Real8(f64::decode(decoder)?)),
        CdfType::Epoch(_) => Ok(CdfType::Epoch(f64::decode(decoder)?)),
        CdfType::Epoch16(_, _) => {
            let v1 = f64::decode(decoder)?;
            let v2 = f64::decode(decoder)?;
            Ok(CdfType::Epoch16(v1, v2))
        },
        CdfType::TimeTt2000(_) => Ok(CdfType::TimeTt2000(i64::decode(decoder)?)),
        CdfType::Byte(_) => Ok(CdfType::Byte(i8::decode(decoder)?)),
        CdfType::Float(_) => Ok(CdfType::Float(f32::decode(decoder)?)),
        CdfType::Double(_) => Ok(CdfType::Double(f64::decode(decoder)?)),
        CdfType::Char(_) => Ok(CdfType::Char(char::decode(decoder)?)),
        CdfType::Uchar(_) => Ok(CdfType::Uchar(char::decode(decoder)?)),
        _ => Err(DecodeError::Other("".to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::Decoder;
    use crate::repr::Encoding;

    #[test]
    fn test_cdftype_int1() -> Result<(), DecodeError> {
        let val = 7i8.to_be_bytes();
        let mut decoder = Decoder {
            reader: val.as_slice(),
            encoding: Encoding::Network,
        };
        assert_eq!(
            CdfType::Int1(7i8),
            decode_cdf_type(&mut decoder, CdfType::Int1(0))?
        );

        Ok(())
    }
}
