use super::Reader;

pub const PREFIX_PATH: u8 = '^' as u8;

/// # PrefixPath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct PrefixPath;

impl From<&PrefixPath> for char {
    fn from(prefix_path: &PrefixPath) -> Self {
        PREFIX_PATH as Self
    }
}

impl From<&[u8]> for PrefixPath {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self
    }
}

impl Reader<'_> for PrefixPath {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| *head == PREFIX_PATH)
    }
}

