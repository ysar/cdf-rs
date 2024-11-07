use std::{io, mem};

use crate::error::DecodeError;
use crate::repr::Encoding;
use crate::types::{Char, Real4, Real8, Uchar, Uint1, Uint2, Uint4, Int1, Int2, Int4, Int8};

/// Trait for decoding a CDF result from a reader.
pub trait Decodable {

    type Value;

    /// Decode a value from the input that implements `io::Read`.
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self::Value, DecodeError>;
}

/// Struct containing the reader and decoding configurations.
pub struct Decoder<R: io::Read> {
    pub reader: R,
    pub encoding: Encoding,
}

macro_rules! impl_decodable_ints {
    ($t:ident, $inner:ident) => {
            impl Decodable for $t {

                type Value = $t;

                fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError> {
                    let mut buffer = [0u8; <$t>::size()];

                    decoder
                        .reader
                        .read_exact(&mut buffer[..])
                        .map_err(|err| DecodeError::Other(format!("{err}")))?;

                    match decoder.encoding {
                        Encoding::Network
                        | Encoding::Sun
                        | Encoding::Next
                        | Encoding::MacPpc
                        | Encoding::Sgi
                        | Encoding::IbmRs
                        | Encoding::ArmBig => Ok($t::from_be_bytes(buffer)),

                        _ => Ok($t::from_le_bytes(buffer)),
                    }
                }
            }
    }
}

impl_decodable_ints!(Uint1, u8);
// Uint2, Uint4);
impl_decodable_ints!(Int1, Int2, Int4, Int8);

macro_rules! impl_decodable_floats {
    ($($t:ident), *) => {
        $(
            impl Decodable for $t {

                type Value = $t;

                fn decode<R: io::Read>(decoder:&mut Decoder<R>) -> Result<Self, DecodeError> {
                    let mut buffer = [0u8; mem::size_of::<$t>()];

                    decoder
                        .reader
                        .read_exact(&mut buffer[..])
                        .map_err(|err| DecodeError::Other(format!("{err}")))?;

                    match decoder.encoding {
                        Encoding::Network
                        | Encoding::Sun
                        | Encoding::Next
                        | Encoding::MacPpc
                        | Encoding::Sgi
                        | Encoding::IbmRs
                        | Encoding::ArmBig => Ok($t::from_be_bytes(buffer)),

                        Encoding::DecStation
                        | Encoding::IbmPc
                        | Encoding::AlphaOsf1
                        | Encoding::AlphaVmsI
                        | Encoding::ArmLittle
                        | Encoding::Ia64VmsI => Ok($t::from_le_bytes(buffer)),

                        _ => Err(DecodeError::Other(
                                "Non-IEEE-574 floating point representation is not supported."
                                .to_string()
                                )),
                    }
                }
            }
        )*
    }
}

impl_decodable_floats!(Real4, Real8);

impl Decodable for Char {
    
    type Value = Char;

    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError> {
        Ok(i8::decode(decoder)? as char)
    }
}

// Tests -------------------------------------------------------------------------------- Tests --
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repr::Encoding;
    use paste::paste;

    macro_rules! test_decode_unsigned_ints {
        ($($t:ident), *) => {
            $(
                paste! {
                    #[test]
                    fn [< test_decode_ $t >]() -> Result<(), DecodeError> {
                        let read_buffer = [< 243 $t >].to_be_bytes();

                        let mut decoder = Decoder {
                            reader: read_buffer.as_slice(),
                            encoding: Encoding::Network,
                        };

                        assert_eq!([< 243 $t >], $t::decode(&mut decoder)?);
                        Ok(())
                    }
                }
            )*
        };
    }

    test_decode_unsigned_ints!(u8, u16, u32);

    macro_rules! test_decode_signed_ints {
        ($($t:ident), *) => {
            $(
                paste! {
                    #[test]
                    fn [< test_decode_ $t >]() -> Result<(), DecodeError> {
                        let read_buffer = (- [< 123 $t >] ).to_be_bytes();

                        let mut decoder = Decoder {
                            reader: read_buffer.as_slice(),
                            encoding: Encoding::Network,
                        };

                        assert_eq!((- [< 123 $t >] ), $t::decode(&mut decoder)?);
                        Ok(())
                    }
                }
            )*
        };
    }

    test_decode_signed_ints!(i8, i16, i32, i64);

    #[test]
    fn test_decode_f32() -> Result<(), DecodeError> {
        let read_buffer = 5.6051938573e-45f32.to_be_bytes();
        let mut decoder = Decoder {
            reader: read_buffer.as_slice(),
            encoding: Encoding::Network,
        };
        assert_eq!(5.6051938573e-45, f32::decode(&mut decoder)?);
        Ok(())
    }

    #[test]
    fn test_decode_f64() -> Result<(), DecodeError> {
        let read_buffer = 1.11253692925360143e-308f64.to_be_bytes();
        let mut decoder = Decoder {
            reader: read_buffer.as_slice(),
            encoding: Encoding::Network,
        };
        assert_eq!(1.11253692925360143e-308f64, f64::decode(&mut decoder)?);
        Ok(())
    }
}
