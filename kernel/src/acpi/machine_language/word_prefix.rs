pub const WORD_PREFIX: u8 = 0x0b;

/// # WordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct WordPrefix;

impl WordPrefix {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for WordPrefix {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), WORD_PREFIX);
        Self
    }
}

