use std::{io, mem};

use crate::error::DecodeError;
use crate::repr::Encoding;

/// Trait for decoding a CDF result from a reader.
pub trait Decode: Sized {
    /// Decode a value from the input that implements `io::Read`.
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError>;
}

/// Struct containing the reader and decoding configurations.
pub struct Decoder<R: io::Read> {
    pub reader: R,
    pub encoding: Encoding,
}

macro_rules! impl_decode_ints {
    ($($t:ident), *) => {
        $(
            impl Decode for $t {
                fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError> {
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

                        _ => Ok($t::from_le_bytes(buffer)),
                    }
                }
            }
        )*
    }
}

impl_decode_ints!(u8, u16, u32);
impl_decode_ints!(i8, i16, i32, i64);

macro_rules! impl_decode_floats {
    ($($t:ident), *) => {
        $(
            impl Decode for $t {
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

impl_decode_floats!(f32, f64);

impl Decode for char {
    fn decode<R: io::Read>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError> {
        Ok(u8::decode(decoder)? as char)
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
