use {
    core::fmt,
    super::{
        ARG_OBJ_MAX,
        ARG_OBJ_MIN,
        ArgObj,
        DataObject,
        ONE_OP,
        Reader,
        WORD_PREFIX,
    },
};

/// # TermArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermArg {
    ExpressionOpcode,
    DataObject(DataObject),
    ArgObj(ArgObj),
    LocalObj,
}

impl fmt::Debug for TermArg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpressionOpcode => write!(formatter, "TermArg::ExpressionOpcode"),
            Self::DataObject(data_object) => formatter
                .debug_tuple("TermArg")
                .field(data_object)
                .finish(),
            Self::ArgObj(arg_obj) => formatter
                .debug_tuple("ArgObj")
                .field(arg_obj)
                .finish(),
            Self::LocalObj => write!(formatter, "TermArg::LocalObj"),
        }
    }
}

impl From<&[u8]> for TermArg {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ONE_OP | WORD_PREFIX => Self::DataObject(aml.into()),
            ARG_OBJ_MIN..=ARG_OBJ_MAX => Self::ArgObj(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for TermArg {
    fn length(&self) -> usize {
        match self {
            Self::ExpressionOpcode => unimplemented!(),
            Self::DataObject(data_object) => data_object.length(),
            Self::ArgObj(arg_obj) => arg_obj.length(),
            Self::LocalObj => unimplemented!(),
        }
    }
}

