use {
    core::fmt,
    super::{
        ArgObj,
        DataObject,
        ExpressionOpcode,
        LocalObj,
        Reader,
    },
};

/// # TermArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermArg {
    ExpressionOpcode(ExpressionOpcode),
    DataObject(DataObject),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl fmt::Debug for TermArg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("TermArg");
        match self {
            Self::ExpressionOpcode(expression_opcode) => debug_tuple.field(expression_opcode),
            Self::DataObject(data_object) => debug_tuple.field(data_object),
            Self::ArgObj(arg_obj) => debug_tuple.field(arg_obj),
            Self::LocalObj(local_obj) => debug_tuple.field(local_obj),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for TermArg {
    fn from(aml: &[u8]) -> Self {
        if ExpressionOpcode::matches(aml) {
            Self::ExpressionOpcode(aml.into())
        } else if DataObject::matches(aml) {
            Self::DataObject(aml.into())
        } else if ArgObj::matches(aml) {
            Self::ArgObj(aml.into())
        } else if LocalObj::matches(aml) {
            Self::LocalObj(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for TermArg {
    fn length(&self) -> usize {
        match self {
            Self::ExpressionOpcode(expression_opcode) => expression_opcode.length(),
            Self::DataObject(data_object) => data_object.length(),
            Self::ArgObj(arg_obj) => arg_obj.length(),
            Self::LocalObj(local_obj) => local_obj.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        ExpressionOpcode::matches(aml)
        || DataObject::matches(aml)
        || ArgObj::matches(aml)
        || LocalObj::matches(aml)
    }
}

