use {
    core::{
        fmt,
        slice,
    },
    super::{
        EXT_OP_PREFIX,
        FIELD_OP,
        METHOD_OP,
        NameSpaceModifierObj,
        NamedObj,
        OP_REGION_OP,
        SCOPE_OP,
    },
};

/// # Object
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Object {
    NameSpaceModifierObj(NameSpaceModifierObj),
    NamedObj(NamedObj),
}

impl Object {
    pub fn length(&self) -> usize {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => name_space_modifier_obj.length(),
            Self::NamedObj(named_obj) => named_obj.length(),
        }
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => formatter
                .debug_tuple("Object")
                .field(name_space_modifier_obj)
                .finish(),
            Self::NamedObj(named_obj) => formatter
                .debug_tuple("Object")
                .field(named_obj)
                .finish(),
        }
    }
}

impl From<&[u8]> for Object {
    fn from(aml: &[u8]) -> Self {
        let mut aml_iterator: slice::Iter<u8> = aml.iter();
        match *aml_iterator.next().unwrap() {
            EXT_OP_PREFIX => match *aml_iterator.next().unwrap() {
                FIELD_OP | OP_REGION_OP => Self::NamedObj(aml.into()),
                unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
            }
            METHOD_OP => Self::NamedObj(aml.into()),
            SCOPE_OP => Self::NameSpaceModifierObj(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

