use {
    core::fmt,
    super::{
        DefName,
        DefScope,
        Reader,
    },
};

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub enum NameSpaceModifierObj {
    DefName(DefName),
    DefScope(DefScope),
}

impl fmt::Debug for NameSpaceModifierObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("NameSpaceModifierObj");
        match self {
            Self::DefName(def_name) => debug_tuple.field(def_name),
            Self::DefScope(def_scope) => debug_tuple.field(def_scope),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for NameSpaceModifierObj {
    fn from(aml: &[u8]) -> Self {
        if DefName::matches(aml) {
            Self::DefName(aml.into())
        } else if DefScope::matches(aml) {
            Self::DefScope(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for NameSpaceModifierObj {
    fn length(&self) -> usize {
        match self {
            Self::DefName(def_name) => def_name.length(),
            Self::DefScope(def_scope) => def_scope.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefName::matches(aml)
        || DefScope::matches(aml)
    }
}

