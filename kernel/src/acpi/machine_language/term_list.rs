use {
    core::fmt,
    super::TermObj,
};

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermList {
    Nothing,
    TermObjTermList {
        term_obj: TermObj,
    },
}

impl fmt::Debug for TermList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => write!(formatter, "TermList::Nothing"),
            Self::TermObjTermList {
                term_obj,
            } => formatter
                .debug_struct("TermList")
                .field("term_obj", term_obj)
                .finish(),
        }
    }
}

impl From<&[u8]> for TermList {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first() {
            Some(first_byte) => {
                let term_obj: TermObj = bytes.into();
                Self::TermObjTermList {
                    term_obj,
                }
            },
            None => Self::Nothing,
        }
    }
}

