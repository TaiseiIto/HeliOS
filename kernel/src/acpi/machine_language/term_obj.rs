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
        SUBTRACT_OP,
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
        if ExpressionOpcode::matches(aml) {
            Self::ExpressionOpcode(aml.into())
        } else if Object::matches(aml) { 
            Self::Object(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
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

    fn matches(aml: &[u8]) -> bool {
        ExpressionOpcode::matches(aml) || Object::matches(aml)
    }
}

