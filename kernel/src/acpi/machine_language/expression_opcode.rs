use {
    core::fmt,
    super::{
        DefToHexString,
        Reader,
        TO_HEX_STRING_OP,
    },
};

/// # ExpressionOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub enum ExpressionOpcode {
    DefToHexString(DefToHexString),
}

impl fmt::Debug for ExpressionOpcode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefToHexString(def_to_hex_string) => formatter
                .debug_tuple("ExpressionOpcode")
                .field(def_to_hex_string)
                .finish(),
        }
    }
}

impl From<&[u8]> for ExpressionOpcode {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            TO_HEX_STRING_OP => Self::DefToHexString(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for ExpressionOpcode {
    pub fn length(&self) -> usize {
        match self {
            Self::DefToHexString(def_to_hex_string) => def_to_hex_string.length(),
        }
    }
}

