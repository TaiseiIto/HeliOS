use {
    alloc::vec::Vec,
    super::{
        ByteData,
        PkgLeadByte,
    },
};

/// # PkgLength
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[derive(Debug)]
pub struct PkgLength {
    pkg_lead_byte: PkgLeadByte,
    bytedata: Vec<ByteData>,
}

impl From<&[u8]> for PkgLength {
    fn from(bytes: &[u8]) -> Self {
        let pkg_lead_byte: PkgLeadByte = bytes.into();
        let bytes: &[u8] = &bytes[pkg_lead_byte.length()..];
        let (bytes, bytedata): (&[u8], Vec<ByteData>) = (0..pkg_lead_byte.bytedata_length())
            .fold((bytes, Vec::new()), |(bytes, mut bytedata), _| {
                let new_bytedata: ByteData = bytes.into();
                let bytes: &[u8] = &bytes[new_bytedata.length()..];
                bytedata.push(new_bytedata);
                (bytes, bytedata)
            });
        Self {
            pkg_lead_byte,
            bytedata,
        }
    }
}

