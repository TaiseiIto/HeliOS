use super::Reader;

const WORD_PREFIX: u8 = 0x0b;

/// # WordPrefix
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct WordPrefix;

impl From<&[u8]> for WordPrefix {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self
    }
}

impl Reader<'_> for WordPrefix {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| *head == WORD_PREFIX)
    }
}

