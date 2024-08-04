use {
    core::{
        fmt,
        slice,
    },
    super::{
        ExpressionOpcode,
        Object,
        Reader,
        StatementOpcode,
    },
};

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermObj {
    ExpressionOpcode(ExpressionOpcode),
    Object(Object),
    StatementOpcode(StatementOpcode),
}

impl fmt::Debug for TermObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("TermObj");
        match self {
            Self::ExpressionOpcode(expression_opcode) => debug_tuple.field(expression_opcode),
            Self::Object(object) => debug_tuple.field(object),
            Self::StatementOpcode(statement_opcode) => debug_tuple.field(statement_opcode),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for TermObj {
    fn from(aml: &[u8]) -> Self {
        if ExpressionOpcode::matches(aml) {
            let (field0, aml): (ExpressionOpcode, &[u8]) = ExpressionOpcode::read(aml);
            Self::ExpressionOpcode(field0)
        } else if Object::matches(aml) { 
            Self::Object(aml.into())
        } else if StatementOpcode::matches(aml) { 
            Self::StatementOpcode(aml.into())
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
            Self::StatementOpcode(statement_opcode) => statement_opcode.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        ExpressionOpcode::matches(aml)
        || Object::matches(aml)
        || StatementOpcode::matches(aml)
    }
}

