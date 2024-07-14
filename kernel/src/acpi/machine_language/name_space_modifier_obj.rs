use {
    core::fmt,
    super::DefScope,
};

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub enum NameSpaceModifierObj {
    DefAlias,
    DefName,
    DefScope {
        def_scope: DefScope,
    },
}

impl fmt::Debug for NameSpaceModifierObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefAlias => write!(formatter, "NameSpaceModifierObj"),
            Self::DefName => write!(formatter, "NameSpaceModifierObj"),
            Self::DefScope {
                def_scope,
            } => formatter
                .debug_struct("NameSpaceModifierObj")
                .field("def_scope", def_scope)
                .finish(),
        }
    }
}

impl From<&[u8]> for NameSpaceModifierObj {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first().unwrap() {
            0x10 => {
                let def_scope: DefScope = bytes.into();
                Self::DefScope {
                    def_scope,
                }
            },
            unknown_byte => panic!("Unknown byte {:#x?}", unknown_byte),
        }
    }
}

