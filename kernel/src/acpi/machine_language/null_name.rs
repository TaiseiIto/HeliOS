/// # NullName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct NullName;

impl NullName {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for NullName {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(*bytes.first().unwrap(), 0x00);
        Self
    }
}
