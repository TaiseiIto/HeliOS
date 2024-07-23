use {
    core::fmt,
    super::{
        ArgObj,
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
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("SimpleName");
        match self {
            Self::NameString(name_string) => debug_tuple.field(name_string),
            Self::ArgObj(arg_obj) => debug_tuple.field(arg_obj),
            Self::LocalObj(local_obj) => debug_tuple.field(local_obj),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for SimpleName {
    fn from(aml: &[u8]) -> Self {
        if NameString::matches(aml) {
            Self::NameString(aml.into())
        } else if ArgObj::matches(aml) {
            Self::ArgObj(aml.into())
        } else if LocalObj::matches(aml) {
            Self::LocalObj(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
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

    fn matches(aml: &[u8]) -> bool {
        NameString::matches(aml)
        || ArgObj::matches(aml)
        || LocalObj::matches(aml)
    }
}

