use super::TermArg;

/// # RegionLen
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct RegionLen(TermArg);

impl RegionLen {
    pub fn length(&self) -> usize {
        self.0.length()
    }
}

impl From<&[u8]> for RegionLen {
    fn from(aml: &[u8]) -> Self {
        Self(aml.into())
    }
}

