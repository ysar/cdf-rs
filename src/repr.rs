use crate::{error::DecodeError, types::CdfInt4};

/// Data Encodings used in CDF (from CDF specification Table 5.11).
#[derive(Debug, PartialEq, Clone)]
pub enum CdfEncoding {
    /// In case the encoding is unspecified.  This will raise an error.
    Unspecified = 0,
    /// eXternal Data Representation
    Network = 1,
    /// Sun Representation
    Sun = 2,
    /// VAX Representation
    Vax = 3,
    /// DECStation Representation
    DecStation = 4,
    /// SGi Representation
    Sgi = 5,
    /// Intel Windows, Linux, MacOS Intel, Solaris Intel
    IbmPc = 6,
    /// IBM RS-6000 Representation
    IbmRs = 7,
    /// Macintosh Power PC Representation
    MacPpc = 9,
    /// HP 9000 Series Representation
    Hp = 11,
    /// NeXT Representation
    Next = 12,
    /// DEC Alpha/OSF1 Representation
    AlphaOsf1 = 13,
    /// DEC Alpha/Open VMS Representation (Double precision floats in D_FLOAT encoding)
    AlphaVmsD = 14,
    /// DEC Alpha/Open VMS Representation (Double precision floats in G_FLOAT encoding)
    AlphaVmsG = 15,
    /// DEC Alpha/Open VMS Representation (Single/Double precision floats in IEEE 754 encoding)
    AlphaVmsI = 16,
    /// ARM little-endian Representation
    ArmLittle = 17,
    /// ARM big-endian Representation
    ArmBig = 18,
    /// Itanium 64 on OpenVMS Representation (Single/Double precision floats in IEEE 754 encoding)
    Ia64VmsI = 19,
    /// Itanium 64 on OpenVMS Representation (Single/Double precision floats in Digital D_FLOAT
    /// encoding)
    Ia64VmsD = 20,
    /// Itanium 64 on OpenVMS Representation (Single/Double precision floats in Digital G_FLOAT
    /// encoding)
    Ia64VmsG = 21,
}

impl CdfEncoding {
    /// Returns the endianness associated with this CDF data encoding.
    pub fn get_endian(&self) -> Result<Endian, DecodeError> {
        match &self {
            CdfEncoding::Network
            | CdfEncoding::Sun
            | CdfEncoding::Next
            | CdfEncoding::MacPpc
            | CdfEncoding::Sgi
            | CdfEncoding::IbmRs
            | CdfEncoding::ArmBig => Ok(Endian::Big),

            CdfEncoding::DecStation
            | CdfEncoding::IbmPc
            | CdfEncoding::AlphaOsf1
            | CdfEncoding::AlphaVmsI
            | CdfEncoding::ArmLittle
            | CdfEncoding::Ia64VmsI => Ok(Endian::Little),

            CdfEncoding::Unspecified => Err(DecodeError::Other(
                "A valid CDF encoding is not read in or is unspecified.".to_string(),
            )),

            _ => Err(DecodeError::Other(
                "Encoding {self:?} not implemented.".to_string(),
            )),
        }
    }
}

impl TryFrom<CdfInt4> for CdfEncoding {
    type Error = DecodeError;
    fn try_from(value: CdfInt4) -> Result<Self, DecodeError> {
        let _value: i32 = value.into();
        match _value {
            0 => Ok(CdfEncoding::Unspecified),
            1 => Ok(CdfEncoding::Network),
            2 => Ok(CdfEncoding::Sun),
            3 => Ok(CdfEncoding::Vax),
            4 => Ok(CdfEncoding::DecStation),
            5 => Ok(CdfEncoding::Sgi),
            6 => Ok(CdfEncoding::IbmPc),
            7 => Ok(CdfEncoding::IbmRs),
            9 => Ok(CdfEncoding::MacPpc),
            11 => Ok(CdfEncoding::Hp),
            12 => Ok(CdfEncoding::Next),
            13 => Ok(CdfEncoding::AlphaOsf1),
            14 => Ok(CdfEncoding::AlphaVmsD),
            15 => Ok(CdfEncoding::AlphaVmsG),
            16 => Ok(CdfEncoding::AlphaVmsI),
            17 => Ok(CdfEncoding::ArmLittle),
            18 => Ok(CdfEncoding::ArmBig),
            19 => Ok(CdfEncoding::Ia64VmsI),
            20 => Ok(CdfEncoding::Ia64VmsD),
            21 => Ok(CdfEncoding::Ia64VmsG),
            v => Err(DecodeError::Other(format!(
                "Invalid encoding integer - {v}."
            ))),
        }
    }
}

/// Enum to handle different endianess.
pub enum Endian {
    Big,
    Little,
}
