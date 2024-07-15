use {
    core::fmt,
    super::{
        DigitChar,
        LeadNameChar,
    },
};

/// # NameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NameChar {
    DigitChar(DigitChar),
    LeadNameChar(LeadNameChar),
}

impl NameChar {
    pub fn length(&self) -> usize {
        match self {
            Self::DigitChar(digit_char) => digit_char.length(),
            Self::LeadNameChar(lead_name_char) => lead_name_char.length(),
        }
    }
}

impl fmt::Debug for NameChar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DigitChar(digit_char) => formatter
                .debug_tuple("NameChar")
                .field(digit_char)
                .finish(),
            Self::LeadNameChar(lead_name_char) => formatter
                .debug_tuple("NameChar")
                .field(lead_name_char)
                .finish(),
        }
    }
}

impl From<&NameChar> for char {
    fn from(name_char: &NameChar) -> Self {
        match name_char {
            NameChar::DigitChar(digit_char) => digit_char.into(),
            NameChar::LeadNameChar(lead_name_char) => lead_name_char.into(),
        }
    }
}

impl From<&[u8]> for NameChar {
    fn from(aml: &[u8]) -> Self {
        let character: char = *aml.first().unwrap() as char;
        match character {
            '0'..='0' => Self::DigitChar(aml.into()),
            'A'..='Z' | '_' => Self::LeadNameChar(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

