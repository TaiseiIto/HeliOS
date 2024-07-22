use {
    core::{
        fmt,
        slice,
    },
    super::{
        EXT_OP_PREFIX,
        ExpressionOpcode,
        FIELD_OP,
        METHOD_OP,
        OP_REGION_OP,
        Object,
        Reader,
        SCOPE_OP,
        TO_BUFFER_OP,
        TO_HEX_STRING_OP,
    },
};

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermObj {
    ExpressionOpcode(ExpressionOpcode),
    Object(Object),
}

impl fmt::Debug for TermObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpressionOpcode(expression_opcode) => formatter
                .debug_tuple("TermObj")
                .field(expression_opcode)
                .finish(),
            Self::Object(object) => formatter
                .debug_tuple("TermObj")
                .field(object)
                .finish(),
        }
    }
}

impl From<&[u8]> for TermObj {
    fn from(aml: &[u8]) -> Self {
        let mut aml_iterator: slice::Iter<u8> = aml.iter();
        match *aml_iterator.next().unwrap() {
            EXT_OP_PREFIX => match *aml_iterator.next().unwrap() {
                FIELD_OP | OP_REGION_OP => Self::Object(aml.into()),
                unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
            },
            METHOD_OP | SCOPE_OP => Self::Object(aml.into()),
            TO_BUFFER_OP | TO_HEX_STRING_OP => Self::ExpressionOpcode(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for TermObj {
    fn length(&self) -> usize {
        match self {
            Self::ExpressionOpcode(expression_opcode) => expression_opcode.length(),
            Self::Object(object) => object.length(),
        }
    }
}

