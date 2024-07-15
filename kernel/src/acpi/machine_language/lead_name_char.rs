/// # LeadNameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(Debug)]
pub struct LeadNameChar(char);

impl LeadNameChar {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&LeadNameChar> for char {
    fn from(lead_name_char: &LeadNameChar) -> Self {
        lead_name_char.0
    }
}

impl From<&[u8]> for LeadNameChar {
    fn from(aml: &[u8]) -> Self {
        let character: char = *aml.first().unwrap() as char;
        match character {
            'A'..='Z' | '_' => Self(character),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

