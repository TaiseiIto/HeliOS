use {
    core::fmt,
    super::{
        DigitChar,
        LeadNameChar,
        Reader,
    },
};

/// # NameChar
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NameChar {
    DigitChar(DigitChar),
    LeadNameChar(LeadNameChar),
}

impl fmt::Debug for NameChar {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("NameChar");
        match self {
            Self::DigitChar(digit_char) => debug_tuple.field(digit_char),
            Self::LeadNameChar(lead_name_char) => debug_tuple.field(lead_name_char),
        };
        debug_tuple.finish()
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
        if DigitChar::matches(aml) {
            Self::DigitChar(aml.into())
        } else if LeadNameChar::matches(aml) {
            Self::LeadNameChar(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for NameChar {
    fn length(&self) -> usize {
        match self {
            Self::DigitChar(digit_char) => digit_char.length(),
            Self::LeadNameChar(lead_name_char) => lead_name_char.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DigitChar::matches(aml) || LeadNameChar::matches(aml)
    }
}

