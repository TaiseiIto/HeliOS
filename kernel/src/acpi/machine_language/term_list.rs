use {
    core::fmt,
    super::term_obj,
};

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Symbol<'a> {
    Nothing,
    TermObjTermList {
        bytes: &'a [u8],
        term_obj: term_obj::Symbol<'a>,
    },
}

impl fmt::Debug for Symbol<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => write!(formatter, "TermList"),
            Self::TermObjTermList {
                bytes,
                term_obj,
            } => formatter.debug_struct("TermList")
                .field("term_obj", term_obj)
                .finish(),
        }
    }
}

impl<'a> From<&'a [u8]> for Symbol<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        match bytes.first() {
            Some(first_byte) => {
                let term_obj: term_obj::Symbol = bytes.into();
                Self::TermObjTermList {
                    bytes,
                    term_obj,
                }
            },
            None => Self::Nothing,
        }
    }
}

