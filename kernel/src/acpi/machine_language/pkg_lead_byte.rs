use bitfield_struct::bitfield;

/// # PkgLeadByte
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[bitfield(u8)]
pub struct PkgLeadByte {
    #[bits(6)]
    nybble: u8,
    #[bits(2)]
    byte_data_count: u8,
}

impl PkgLeadByte {
    pub fn byte_data_length(&self) -> usize {
        self.byte_data_count() as usize
    }

    pub fn length(&self) -> usize {
        1
    }

    pub fn pkg_length(&self) -> usize {
        self.nybble() as usize
    }
}

impl From<&[u8]> for PkgLeadByte {
    fn from(bytes: &[u8]) -> Self {
        (*bytes.first().unwrap()).into()
    }
}

