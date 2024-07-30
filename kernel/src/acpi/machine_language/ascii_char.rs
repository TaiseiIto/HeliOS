use {
    core::ops::RangeInclusive,
    super::Reader,
};

const ASCII_CHAR_RANGE: RangeInclusive<u8> = 0x01..=0x7f;

/// # AsciiChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct AsciiChar(char);

impl From<&[u8]> for AsciiChar {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self(*aml.first().unwrap() as char)
    }
}

impl Reader<'_> for AsciiChar {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
            .first()
            .is_some_and(|head| ASCII_CHAR_RANGE.contains(head))
    }
}

