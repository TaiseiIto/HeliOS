use {
    core::{
        fmt,
        slice,
    },
    super::{
        NameSpaceModifierObj,
        NamedObj,
        Reader,
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
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("Object");
        match self {
            Self::NameSpaceModifierObj(name_space_modifier_obj) => debug_tuple.field(name_space_modifier_obj),
            Self::NamedObj(named_obj) => debug_tuple.field(named_obj),
        };
        debug_tuple.finish()
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

