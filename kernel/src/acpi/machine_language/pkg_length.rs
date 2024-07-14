use super::PkgLeadByte;

/// # PkgLength
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.4 Package Length Encoding
#[derive(Debug)]
pub struct PkgLength {
    pkg_lead_byte: PkgLeadByte,
}

impl From<&[u8]> for PkgLength {
    fn from(bytes: &[u8]) -> Self {
        let pkg_lead_byte: PkgLeadByte = bytes.into();
        Self {
            pkg_lead_byte,
        }
    }
}

