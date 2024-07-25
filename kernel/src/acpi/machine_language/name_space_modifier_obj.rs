use {
    core::fmt,
    super::{
        DefScope,
        Reader,
    },
};

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub enum NameSpaceModifierObj {
    DefScope(DefScope),
}

impl fmt::Debug for NameSpaceModifierObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefScope(def_scope) => formatter
                .debug_tuple("NameSpaceModifierObj")
                .field(def_scope)
                .finish(),
        }
    }
}

impl From<&[u8]> for NameSpaceModifierObj {
    fn from(aml: &[u8]) -> Self {
        if DefScope::matches(aml) {
            Self::DefScope(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for NameSpaceModifierObj {
    fn length(&self) -> usize {
        match self {
            Self::DefScope(def_scope) => def_scope.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefScope::matches(aml)
    }
}

