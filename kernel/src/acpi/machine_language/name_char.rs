use super::{
    DigitChar,
    LeadNameChar,
};

/// # NameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum NameChar {
    DigitChar(DigitChar),
    LeadNameChar(LeadNameChar),
}

impl From<&NameChar> for char {
    fn from(name_char: &NameChar) -> Self {
        match name_char {
            NameChar::DigitChar(digit_char) => digit_char.into(),
            NameChar::LeadNameChar(lead_name_char) => lead_name_char.into(),
        }
    }
}

