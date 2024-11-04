/// Data Encodings used in CDF (from CDF specification Table 5.11).
pub enum Encoding {
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
    Ppc = 9,
    /// HP 9000 Series Representation
    Hp = 11,
    /// NeXT Representation
    Next = 12,
    /// DEC Alpha/OSF1 Representation
    AlphaOsf1 = 13,
    /// DEC Alpha/Open VMS Representation (Double precision floats in D_FLOAT
    /// encoding)
    AlphaVmsD = 14,
    /// DEC Alpha/Open VMS Representation (Double precision floats in G_FLOAT
    /// encoding)
    AlphaVmsG = 15,
    /// DEC Alpha/Open VMS Representation (Single/Double precision floats in 
    /// IEEE 754 encoding)
    AlphaVmsI = 16,
    /// ARM little-endian Representation
    ArmLittle = 17,
    /// ARM big-endian Representation
    ArmBig = 18,
    /// Itanium 64 on OpenVMS Representation (Single/Double precision floats 
    /// in IEEE 754 encoding)
    Ia64VmsI = 19,
    /// Itanium 64 on OpenVMS Representation (Single/Double precision floats 
    /// in Digital D_FLOAT encoding)
    Ia64VmsD = 20,
    /// Itanium 64 on OpenVMS Representation (Single/Double precision floats
    /// in Digital G_FLOAT encoding)
    Ia64VmsG = 21,
}
