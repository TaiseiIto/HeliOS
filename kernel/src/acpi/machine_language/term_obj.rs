use {
    core::fmt,
    super::{
        Object,
        SCOPE_OP,
    },
};

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermObj {
    ExpressionOpcode,
    Object(Object),
    StatementOpcode,
}

impl TermObj {
    pub fn length(&self) -> usize {
        match self {
            Self::ExpressionOpcode => unimplemented!(),
            Self::Object(object) => object.length(),
            Self::StatementOpcode => unimplemented!(),
        }
    }
}

impl fmt::Debug for TermObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpressionOpcode => write!(formatter, "TermObj"),
            Self::Object(object) => formatter
                .debug_struct("TermObj")
                .field("object", object)
                .finish(),
            Self::StatementOpcode => write!(formatter, "TermObj"),
        }
    }
}

impl From<&[u8]> for TermObj {
    fn from(bytes: &[u8]) -> Self {
        match *bytes.first().unwrap() {
            SCOPE_OP => {
                let object: Object = bytes.into();
                Self::Object(object)
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

