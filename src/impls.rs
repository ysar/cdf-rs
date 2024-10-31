use std::{io, mem};

use crate::error::DecodeError;
use crate::traits::{Decode, Encode};

// A small macro for implementing the Decode and Encode traits for primitive types.
#[cfg(target_endian = "little")]
macro_rules! impl_encode_decode_primitives {
    ($($t:ident), *) => {
        $(
            impl<R> Decode<R> for $t
            where
                R: io::Read,
            {
                type Output = $t;

                fn decode(mut reader: R) -> Result<Self::Output, crate::error::DecodeError> {

                    let mut buffer = [0u8; mem::size_of::<$t>()];

                    reader.read_exact(&mut buffer[..])
                        .map_err(|err| DecodeError::Other(format!("{err}")))?;

                    Ok($t::from_le_bytes(buffer))
                }
            }

            //impl<W> Encode<W> for $t
            //where
            //    W: io::Write
            //{
            //    type Input = $t;
            //
            //    fn encode() -> Result<
            //}
        )*
    }
}

impl_encode_decode_primitives!(u8, u16, u32, u64, u128);
impl_encode_decode_primitives!(i8, i16, i32, i64, i128);
impl_encode_decode_primitives!(f32, f64);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_decode_primitives() -> Result<(), DecodeError> {
        assert_eq!(243u8, u8::decode(243u8.to_ne_bytes().as_slice())?);
        assert_eq!(243u16, u16::decode(243u16.to_ne_bytes().as_slice())?);
        assert_eq!(243u32, u32::decode(243u32.to_ne_bytes().as_slice())?);
        assert_eq!(243u64, u64::decode(243u64.to_ne_bytes().as_slice())?);

        assert_eq!(-4i8, i8::decode((-4i8).to_ne_bytes().as_slice())?);
        assert_eq!(-4i16, i16::decode((-4i16).to_ne_bytes().as_slice())?);
        assert_eq!(-4i32, i32::decode((-4i32).to_ne_bytes().as_slice())?);
        assert_eq!(-4i64, i64::decode((-4i64).to_ne_bytes().as_slice())?);
        Ok(())
    }
}
