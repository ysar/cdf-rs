
/// A CDF file contains 2 or more internal records that organize the contents of the CDF.
pub enum InternalRecordTypes {
    
    /// Unused Internal Record
    UIR = -1,

    /// CDF Descriptor Record. All CDF files must contain this field.
    CDR = 1,
    
    /// Global Descriptor Record. All CDF files must contain this field.
    GDR = 2,

    /// rVariable Descriptor Record
    RVDR = 3,

    /// Attribute Descriptor Record
    ADR = 4,

    /// Attribute g/rEntry Descriptor Record
    AgrEDR = 5,

    /// Variable Index Record
    VXR = 6,

    /// Variable Values Record
    VVR = 7,

    /// zVariable Descriptor Record
    ZVDR = 8,

    /// Attribute zEntry Descriptor Record
    AzEDR = 9,

    /// Compressed CDF Record
    CCR = 10,

    /// Compressed Parameters Record
    CPR = 11,

    /// Sparseness Parameters Record
    SPR = 12,

    /// Compressed Variable Values Record
    CVVR = 13,

    // The MD5 checksum is not considered a CDF Internal Record. It is optional and occupies 
    // 16-bytes at the end of the CDF file. These 16-bytes are not included in the eof field in 
    // the global descriptor record (GDR), which typically represents the CDF file size.

}

#[cfg(test)]
mod tests {

    use std::fs::File;
    use std::io::Read;
    use std::path::PathBuf;

    #[test]
    fn read_magic_number() {
        
        let path_test_file: PathBuf = [
            env!("CARGO_MANIFEST_DIR"),
            "tests",
            "data",
            "test_alltypes.cdf",
        ].iter().collect();
        
        let mut file = File::open(path_test_file).unwrap();
        
        let mut buf = [0u8; 4];
        file.read_exact(&mut buf[..]).unwrap();
        let magic_number = u32::from_be_bytes(buf);

        assert_eq!(magic_number, 0xcdf30001u32);
    }
}
