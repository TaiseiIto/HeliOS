use {
    core::fmt,
    super::object,
};

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Symbol {
    ExpressionOpcode,
    Object {
        object: object::Symbol,
    },
    StatementOpcode,
}

impl fmt::Debug for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpressionOpcode => write!(formatter, "TermObj"),
            Self::Object {
                object,
            } => formatter
                .debug_struct("TermObj")
                .field("object", object)
                .finish(),
            Self::StatementOpcode => write!(formatter, "TermObj"),
        }
    }
}

impl From<&[u8]> for Symbol {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first() {
            Some(first_byte) => match first_byte {
                0x10 => {
                    let object: object::Symbol = bytes.into();
                    Self::Object {
                        object,
                    }
                },
                _ => unimplemented!(),
            }
            None => unimplemented!(),
        }
    }
}

