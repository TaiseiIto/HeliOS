use {
    core::fmt,
    super::{
        DefScope,
        SCOPE_OP,
    },
};

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub enum NameSpaceModifierObj {
    DefAlias,
    DefName,
    DefScope(DefScope),
}

impl NameSpaceModifierObj {
    pub fn length(&self) -> usize {
        match self {
            Self::DefAlias => unimplemented!(),
            Self::DefName => unimplemented!(),
            Self::DefScope(def_scope) => def_scope.length(),
        }
    }
}

impl fmt::Debug for NameSpaceModifierObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefAlias => write!(formatter, "NameSpaceModifierObj"),
            Self::DefName => write!(formatter, "NameSpaceModifierObj"),
            Self::DefScope(def_scope) => formatter
                .debug_tuple("NameSpaceModifierObj")
                .field(def_scope)
                .finish(),
        }
    }
}

impl From<&[u8]> for NameSpaceModifierObj {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            SCOPE_OP => Self::DefScope(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

