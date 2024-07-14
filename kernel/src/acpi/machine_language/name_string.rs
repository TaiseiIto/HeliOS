/// # DefScope
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct NameString;

impl From<&[u8]> for NameString {
    fn from(_: &[u8]) -> Self {
        Self
    }
}

