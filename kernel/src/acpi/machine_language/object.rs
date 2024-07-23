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
        Reader,
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
        if NameSpaceModifierObj::matches(aml) {
            Self::NameSpaceModifierObj(aml.into())
        } else if NamedObj::matches(aml) {
            Self::NamedObj(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for Object {
    fn length(&self) -> usize {
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => name_space_modifier_obj.length(),
            Self::NamedObj(named_obj) => named_obj.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        NameSpaceModifierObj::matches(aml) || NamedObj::matches(aml)
    }
}

