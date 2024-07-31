use super::{
    Reader,
    WordData,
};

/// # Timeout
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct Timeout(WordData);

impl From<&[u8]> for Timeout {
     fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (word_data, aml): (WordData, &[u8]) = WordData::read(aml);
        Self(word_data)
     }
}

impl Reader<'_> for Timeout {
    fn length(&self) -> usize {
        self.0.length()
    }

    fn matches(aml: &[u8]) -> bool {
        WordData::matches(aml)
    }
}

