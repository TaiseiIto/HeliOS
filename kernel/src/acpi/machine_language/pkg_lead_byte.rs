use {
    bitfield_struct::bitfield,
    super::Reader,
};

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

    pub fn pkg_length(&self) -> usize {
        self.nybble() as usize
    }
}

impl From<&[u8]> for PkgLeadByte {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        (*aml.first().unwrap()).into()
    }
}

impl Reader<'_> for PkgLeadByte {
    fn length(&self) -> usize {
        1
    }

    fn matches(_aml: &[u8]) -> bool {
        true
    }
}

