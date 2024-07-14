use {
    core::fmt,
    super::def_scope,
};

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub enum Symbol {
    DefAlias,
    DefName,
    DefScope {
        def_scope: def_scope::Symbol,
    },
}

impl fmt::Debug for Symbol {
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

impl From<&[u8]> for Symbol {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first() {
            Some(first_byte) => match first_byte {
                0x10 => {
                    let def_scope: def_scope::Symbol = bytes.into();
                    Self::DefScope {
                        def_scope,
                    }
                },
                unknown_byte => panic!("Unknown byte {:#x?}", unknown_byte),
            }
            None => unimplemented!(),
        }
    }
}

