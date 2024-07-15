use core::fmt;

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub enum NamedObj {
    DefOpRegion,
}

impl fmt::Debug for NamedObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl From<&[u8]> for NamedObj {
    fn from(aml: &[u8]) -> Self {
        unimplemented!()
    }
}

