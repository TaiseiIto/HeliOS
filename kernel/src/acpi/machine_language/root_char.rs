use super::Reader;

pub const ROOT_CHAR: u8 = '\\' as u8;

/// # RootChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct RootChar;

impl From<&RootChar> for char {
    fn from(root_char: &RootChar) -> Self {
        ROOT_CHAR as Self
    }
}

impl From<&[u8]> for RootChar {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self
    }
}

impl Reader<'_> for RootChar {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| *head == ROOT_CHAR)
    }
}

