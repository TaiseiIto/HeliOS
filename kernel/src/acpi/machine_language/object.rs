use {
    core::fmt,
    super::{
        NameSpaceModifierObj,
        SCOPE_OP,
    },
};

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Object {
    NameSpaceModifierObj(NameSpaceModifierObj),
    NamedObj,
}

impl Object {
    pub fn length(&self) -> usize {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => name_space_modifier_obj.length(),
            Self::NamedObj => unimplemented!(),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => formatter
                .debug_struct("Object")
                .field("name_space_modifier_obj", name_space_modifier_obj)
                .finish(),
            Self::NamedObj => write!(formatter, "Object"),
        }
    }
}

impl From<&[u8]> for Object {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            SCOPE_OP => {
                let name_space_modifier_obj: NameSpaceModifierObj = aml.into();
                Self::NameSpaceModifierObj(name_space_modifier_obj)
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

