/// # DigitChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct DigitChar(char);

impl DigitChar {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for DigitChar {
    fn from(aml: &[u8]) -> Self {
        let character: char = *aml.first().unwrap() as char;
        assert!(('0'..='9').contains(&character));
        Self(character)
    }
}

