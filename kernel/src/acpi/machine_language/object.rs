use {
    core::fmt,
    super::NameSpaceModifierObj,
};

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Object {
    NameSpaceModifierObj {
        name_space_modifier_obj: NameSpaceModifierObj,
    },
    NamedObj,
}

impl fmt::Debug for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSpaceModifierObj {
                name_space_modifier_obj,
            } => formatter
                .debug_struct("Object")
                .field("name_space_modifier_obj", name_space_modifier_obj)
                .finish(),
            Self::NamedObj => write!(formatter, "Object"),
        }
    }
}

impl From<&[u8]> for Object {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first() {
            Some(first_byte) => match first_byte {
                0x10 => {
                    let name_space_modifier_obj: NameSpaceModifierObj = bytes.into();
                    Self::NameSpaceModifierObj {
                        name_space_modifier_obj,
                    }
                },
                unknown_byte => panic!("Unknown byte {:#x?}", unknown_byte),
            }
            None => unimplemented!(),
        }
    }
}

