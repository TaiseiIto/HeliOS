use {
    core::fmt,
    super::{
        DefSizeOf,
        DefSubtract,
        DefToBuffer,
        DefToHexString,
        Reader,
        SIZE_OF_OP,
        SUBTRACT_OP,
        TO_BUFFER_OP,
        TO_HEX_STRING_OP,
    },
};

/// # ExpressionOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub enum ExpressionOpcode {
    DefSizeOf(DefSizeOf),
    DefSubtract(DefSubtract),
    DefToBuffer(DefToBuffer),
    DefToHexString(DefToHexString),
}

impl fmt::Debug for ExpressionOpcode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefSizeOf(def_size_of) => formatter
                .debug_tuple("ExpressionOpcode")
                .field(def_size_of)
                .finish(),
            Self::DefSubtract(def_subtract) => formatter
                .debug_tuple("ExpressionOpcode")
                .field(def_subtract)
                .finish(),
            Self::DefToBuffer(def_to_buffer) => formatter
                .debug_tuple("ExpressionOpcode")
                .field(def_to_buffer)
                .finish(),
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
            SIZE_OF_OP => Self::DefSizeOf(aml.into()),
            SUBTRACT_OP => Self::DefSubtract(aml.into()),
            TO_BUFFER_OP => Self::DefToBuffer(aml.into()),
            TO_HEX_STRING_OP => Self::DefToHexString(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for ExpressionOpcode {
    fn length(&self) -> usize {
        match self {
            Self::DefSizeOf(def_size_of) => def_size_of.length(),
            Self::DefSubtract(def_subtract) => def_subtract.length(),
            Self::DefToBuffer(def_to_buffer) => def_to_buffer.length(),
            Self::DefToHexString(def_to_hex_string) => def_to_hex_string.length(),
        }
    }
}

