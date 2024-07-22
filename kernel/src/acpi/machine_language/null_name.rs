use super::Reader;

pub const NULL_NAME: u8 = 0x00;

/// # NullName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct NullName;

impl From<&[u8]> for NullName {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), NULL_NAME);
        Self
    }
}

impl Reader<'_> for NullName {
    fn length(&self) -> usize {
        1
    }
}

