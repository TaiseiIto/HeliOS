pub const ROOT_CHAR: u8 = 0x5c;

/// # RootChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct RootChar;

impl RootChar {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for RootChar {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), ROOT_CHAR);
        Self
    }
}

