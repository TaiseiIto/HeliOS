use super::Reader;

/// # DigitChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct DigitChar(char);

impl From<&DigitChar> for char {
    fn from(digit_char: &DigitChar) -> char {
        digit_char.0
    }
}

impl From<&[u8]> for DigitChar {
    fn from(aml: &[u8]) -> Self {
        let character: char = *aml.first().unwrap() as char;
        assert!(('0'..='9').contains(&character));
        Self(character)
    }
}

impl Reader<'_> for DigitChar {
    fn length(&self) -> usize {
        1
    }
}

