use bitfield_struct::bitfield;

/// # PkgLeadByte
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[bitfield(u8)]
pub struct PkgLeadByte {
    #[bits(6)]
    package_length: u8,
    #[bits(2)]
    bytedata_count: u8,
}

impl From<&[u8]> for PkgLeadByte {
    fn from(bytes: &[u8]) -> Self {
        (*bytes.first().unwrap()).into()
    }
}

