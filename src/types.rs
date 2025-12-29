/// The CDF format supports different data types like ints and floats of
/// different sizes. This module defines these fundamental types (CdfXXXX) and
/// there conversions from and into byte arrays and native Rust types.
use crate::decode::{Decodable, Decoder};
use crate::error::DecodeError;
use crate::repr::Endian;
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
            // Required method
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
                write!(f, "{}", self.0)
            }
        }
        impl Debug for $cdf_type {
            // Required method
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
impl_cdf_type!(CdfChar, i8);
impl_cdf_type!(CdfUchar, u8);
// pub type CdfFloat = CdfReal4;
// pub type CdfDouble = CdfReal8;

// pub struct CdfEpoch16(CdfReal8, CdfReal8);

// impl CdfEpoch16 {
//     pub const fn size() -> usize {
//         16
//     }
// }

// Each CdfType is encoded/decoded in little or big-endian format depending on the type of
// CdfEncoding that is used.

macro_rules! impl_decodable {
    ($($t:ident), *) => {
        $(
            impl Decodable for $t {

                type Value = $t;

                fn decode<R>(decoder: &mut Decoder<R>) -> Result<Self, DecodeError>
                where
                    R: io::Read + io::Seek
                {
                    let mut buffer = [0u8; <$t>::size()];

                    decoder
                        .reader
                        .read_exact(&mut buffer[..])
                        .map_err(|err| DecodeError::Other(format!("{err}")))?;

                    match decoder.endianness {
                        Endian::Big => Ok($t::from_be_bytes(buffer)),
                        Endian::Little => Ok($t::from_le_bytes(buffer)),
                    }
                }
            }
        )*
    }
}

impl_decodable!(CdfUint1, CdfUint2, CdfUint4);
impl_decodable!(CdfInt1, CdfInt2, CdfInt4, CdfInt8);
impl_decodable!(CdfTimeTt2000, CdfByte, CdfChar, CdfUchar);
impl_decodable!(CdfReal4, CdfReal8);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decode::Decoder;
    use crate::error::CdfError;
    use crate::repr::Endian;
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
                    let mut decoder = Decoder::new(io::Cursor::new(y.as_slice()), Endian::Big, None)?;
                    assert_eq!($t1(x), $t1::decode(&mut decoder)?);

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
