use {
    core::fmt,
    super::{
        ARG_OBJ_MAX,
        ARG_OBJ_MIN,
        ArgObj,
        LOCAL_OBJ_MAX,
        LOCAL_OBJ_MIN,
        LocalObj,
        NameString,
        Reader,
    },
};

/// # SimpleName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum SimpleName {
    NameString(NameString),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

impl fmt::Debug for SimpleName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameString(name_string) => formatter
                .debug_tuple("SimpleName")
                .field(name_string)
                .finish(),
            Self::ArgObj(arg_obj) => formatter
                .debug_tuple("SimpleName")
                .field(arg_obj)
                .finish(),
            Self::LocalObj(local_obj) => formatter
                .debug_tuple("SimpleName")
                .field(local_obj)
                .finish(),
        }
    }
}

impl From<&[u8]> for SimpleName {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ARG_OBJ_MIN..=ARG_OBJ_MAX => Self::ArgObj(aml.into()),
            LOCAL_OBJ_MIN..=LOCAL_OBJ_MAX => Self::LocalObj(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for SimpleName {
    fn length(&self) -> usize {
        match self {
            Self::NameString(name_string) => name_string.length(),
            Self::ArgObj(arg_obj) => arg_obj.length(),
            Self::LocalObj(local_obj) => local_obj.length(),
        }
    }
}

